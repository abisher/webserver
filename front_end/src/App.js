import React, {Component} from "react";
import axios from "axios";
import ToDoItem from "./components/ToDoItem";
import CreateToDoItem from "./components/CreateToDoItem";


class App extends Component {
    state = {
        "pending_items": [],
        "done_items": [],
        "pending_items_count": 0,
        "done_items_count": 0
    }

    // makes the API call
    getItems() {
        axios.get("http://127.0.0.1:8080/v1/item/get",
            {headers: {"token": "some_token"}})
            .then(response => {
                let pendingItems = response.data["pending_items"]
                let done_items = response.data["done_items"]

                this.setState({
                    "pending_items": this.processItemValues(pendingItems),
                    "done_items": this.processItemValues(done_items),
                    "pending_items_count": response.data["pending_item_count"],
                    "done_items_count": response.data["done_item_count"]
                })
            })
    }

    // ensures the API call is updated than mounted
    componentDidMount() {
        this.getItems();
    }

    // converts items from API to HTML
    processItemValues(items) {
        let itemList = [];
        items.forEach((item, _) => {
            console.log(item.status)
            itemList.push(
                <ToDoItem key={item.status + item.title}
                          title={item.title}
                          status={item.status}
                          passBackResponse={this.handleReturnedState}
                />)
        })

        return itemList
    }

    handleReturnedState = (response) => {
        let pendingItems = response.data["pending_items"]
        let doneItems = response.data["done_items"]
        this.setState({
            "pending_items": this.processItemValues(pendingItems),
            "done_items": this.processItemValues(doneItems),
            "pending_items_count": response.data["pendingItemsCount"],
            "done_items_count": response.data["doneItemsCount"]
        })
    }


    render() {
        return (
            <div className="App">
                <h1>Done Items</h1>
                <p>done item count: {this.state.done_items_count}</p>
                {this.state.done_items}
                <h1>Pending Items</h1>
                <p>pending item count: {this.state.pending_items_count}</p>
                {this.state.pending_items}
                <CreateToDoItem passBackResponse={this.handleReturnedState}/>
            </div>
        )
    }
}

export default App; 