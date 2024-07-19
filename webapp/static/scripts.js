const $table_cards = document.getElementById("table").children
const $cards = document.getElementById("handcards").children
const $debug = document.getElementById("debug_div");
/** @type {WebSocket | null} */
const socket = new WebSocket(`ws://${window.location.host}/ws`)

socket.onopen = () => {
    for (let i = 0; i < $cards.length; i++) {
    $cards[i].addEventListener('click', 
        () => {
            socket.send(`${i}`)
        })
        
    }
}

function render_cards(table, elems) {
    for (let i = 0; i < elems.length; i++) {
        table[i].src = `assets/fronts/${elems[i]}.svg`
        table[i].style.visibility = "visible"
    }
    for (let i = elems.length; i < table.length; i++) {
        table[i].style.visibility = "hidden"
        table[i].src = `assets/backs/blue.svg`
    }
}

socket.onmessage = (ev) => {
    $debug.innerText = ev.data
    if (ev.data === "Select card") {
        return
    }
    const msg = JSON.parse(ev.data)
    if (typeof msg === "object") {
        render_cards($cards, msg.hand)
        render_cards($table_cards, msg.table)
    }
}
