class Node:
    def __init__(self, key, node=None):
        self.key = key
        self.next = node

    def __repr__(self):
        return f"Node(key={self.key})"

class LinkedList:
    def __init__(self):
        self.head = Node(None)
        self.z = Node(None)
        self.head.next = self.z
        self.z.next = self.z

    def delete_next(self, node):
        node.next = node.next.next

    def insert_after(self, value, node):
        new_node = Node(value, node.next)
        node.next = new_node
        return new_node

    def __repr__(self):
        result = ""
        current_node = self.head.next
        index = 0
        while current_node != self.z:
            result += f"{index}: {current_node}\n"
            current_node = current_node.next
            index += 1
        return result
    
def main():
    linked_list = LinkedList()
    node_one = linked_list.insert_after(5, linked_list.head)
    node_two = linked_list.insert_after(10, node_one)
    node_three = linked_list.insert_after(15, node_two)

    print(f"{linked_list}")
    print(f"Delete {node_two}\n")

    linked_list.delete_next(node_one)
    print(f"{linked_list}")
    
if __name__ == "__main__":
    main()
