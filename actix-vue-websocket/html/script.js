const app = new Vue({
    el: "#app",
    data: {
        url_ws: "ws://localhost:8090/ws/",
        message: "",
        logs: [],
        status: "disconnected"
    },

    methods: {
        connect() {
            // https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/WebSocket
            let wsUri = (window.location.protocol=='https:'&&'wss://'||'ws://')+window.location.host + '/ws/';
            // console.log(wsUri);
            console.log(this.url_ws);
            this.socket = new WebSocket(wsUri);
            this.socket.onopen = () => {
                this.status = "connected";
                this.logs.push({ event: "Connected to", data: this.url_ws });


                this.socket.onmessage = ({ data }) => {
                    this.logs.push({ event: "Recieved message", data });
                };
            };
        },
        disconnect() {
            this.socket.close();
            this.status = "disconnected";
            this.logs = [];
        },
        sendMessage(e) {
            this.socket.send(this.message);
            this.logs.push({ event: "Sent message", data: this.message });
            this.message = "";
        },
        sendBinary(e) {
            // create an ArrayBuffer with a size in bytes
            let buffer = new ArrayBuffer(8);
            this.socket.send(buffer);
            this.logs.push({ event: "Sent message", data: this.message });
            this.message = "";
        }
    }
});