use std::ops::Deref;
use std::sync::{
    mpsc::{channel, Sender},
    Condvar, Mutex,
};
use std::thread;

use actix::prelude::*;
use actix_files::NamedFile;
use actix_web::{
    dev::ServerHandle, middleware, rt, web, web::Data, App, Error, HttpRequest, HttpResponse,
    HttpServer, Responder,
};
use actix_web_actors::ws;

use crate::MutCondVarPair;
pub struct MyWebSocket {
    mcv: MutCondVarPair,
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        log::info!("WS: {msg:?}");
        match msg {
            Ok(ws::Message::Text(text)) => {
                log::info!("Received {text}");
                if let Ok(num) = text.parse::<usize>() {
                    let mut index = self.mcv.mutex().lock().unwrap();
                    if index.is_none() {
                        *index = Some(num);
                        self.mcv.cond_var().notify_one();
                    }
                }
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

async fn index() -> impl Responder {
    NamedFile::open_async("./webapp/static/index.html")
        .await
        .unwrap()
}

pub struct TextMessage {
    pub msg: String,
}

impl Message for TextMessage {
    type Result = ();
}

impl Handler<TextMessage> for MyWebSocket {
    type Result = ();
    fn handle(&mut self, msg: TextMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.msg);
    }
}

async fn echo_ws(
    req: HttpRequest,
    stream: web::Payload,
    mutex: Data<(Mutex<Option<usize>>, Condvar)>,
    addr_out: Data<(Mutex<Option<Addr<MyWebSocket>>>, Condvar)>,
) -> Result<HttpResponse, Error> {
    let (addr, resp) = ws::WsResponseBuilder::new(
        MyWebSocket {
            mcv: MutCondVarPair {
                mutex_and_cond_var: mutex.clone().into_inner(),
            },
        },
        &req,
        stream,
    )
    .start_with_addr()?;
    let inner = addr_out.into_inner();
    *inner.0.lock().unwrap() = Some(addr);
    inner.1.notify_one();
    Ok(resp)
}

async fn run_server(
    tx: Sender<ServerHandle>,
    port: u16,
    mutex: Data<(Mutex<Option<usize>>, Condvar)>,
    addr: Data<(Mutex<Option<Addr<MyWebSocket>>>, Condvar)>,
) -> std::io::Result<()> {
    log::info!("starting HTTP server at http://localhost:{port}");

    let srv = HttpServer::new(move || {
        App::new()
            .service(web::resource("/").to(index))
            .service(actix_files::Files::new("/assets", "./webapp/assets"))
            .service(
                web::resource("/ws")
                    .app_data(mutex.clone())
                    .app_data(addr.clone())
                    .route(web::get().to(echo_ws)),
            )
            .service(actix_files::Files::new("/", "./webapp/static"))
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", port))?
    .run();

    tx.send(srv.handle()).unwrap();
    srv.await
}

pub fn start_server(port: u16) -> (ServerHandle, MutCondVarPair, Addr<MyWebSocket>) {
    let (tx, rx) = channel();
    let mutex = Data::new((Mutex::new(None), Condvar::new()));
    let mutex_clone = mutex.clone();
    let addr = Data::new((Mutex::new(None), Condvar::new()));
    let addr_clone = addr.clone();
    let lock = addr.0.lock().unwrap();
    thread::spawn(move || {
        let server_future = run_server(tx, port, mutex_clone, addr_clone);
        rt::System::new().block_on(server_future)
    });
    log::info!("Waiting for address");
    let lock = addr.1.wait_while(lock, |opt| opt.is_none()).unwrap();
    log::info!("Done waiting");
    (
        rx.recv().unwrap(),
        MutCondVarPair {
            mutex_and_cond_var: mutex.into_inner(),
        },
        lock.deref().clone().unwrap(),
    )
}
