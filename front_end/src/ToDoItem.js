import React, {Component} from "react";
import axios from "axios";

class ToDoItem extends Component {
    state = {
        "title": this.props.title,
        "status": this.props.status,
        "button": this.processStatus(this.props.status)
    }

    processStatus(status) {
    }

    inverseStatus(status) {

    }

    sendRequest = () => {

    }

    render() {
        return
    }
}


export default ToDoItem;