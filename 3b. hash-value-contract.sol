// SPDX-License-Identifier: GPL-3.0
pragma solidity >=0.4.16 <0.9.0;

contract TransactionHashStore {

    string question_value_hash = "5feceb66ffc86f38d952786c6d696c79c2dbc239dd4e91b46729d73a27fb57e9";

    function set(string question_value_hash) public {
        question_value_hash = x;
    }

    function get() public view returns (string) {
        return question_value_hash;
    }
}
