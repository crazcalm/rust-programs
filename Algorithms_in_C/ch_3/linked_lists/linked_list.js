class Node {
    constructor(key, node) {
        this.key = key;
        this.node = node;
    }
    print() {
        console.log(`Node(${this.key})`);
    }
}

class LinkedList {
    constructor() {
        this.head = new Node(null, null);
        this.z = new Node(null, null);
        this.head.next = this.z;
        this.z.next = this.z; // Sentinel node
    }

    delete_next(node) {
        node.next = node.next.next;
    }

    insert_after(value, node) {
        const newNode = new Node(value, node.next);
        node.next = newNode;
        return newNode;
    }
    print() {
        let current = this.head.next;
        while (current != null) {
            current.print();
            current = current.next;
        }
    }
}


function main() {
    const linkedList = new LinkedList();
    nodeOne = linkedList.insert_after(5, linkedList.head);
    nodeTwo = linkedList.insert_after(10, nodeOne);
    nodeThree = linkedList.insert_after(15, nodeTwo);

    console.log("Print linkedList")
    linkedList.print()

    console.log(`delete ${nodeTwo.key}`);
    linkedList.delete_next(nodeOne);
    console.log("\nPrint linkedList");
    linkedList.print();
}

main();