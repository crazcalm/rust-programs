class Node:
    def __init__(self, key, next=None):
        self.key = key
        self.next = next


def main():
    node = Node(1, None)

    # Capture user input
    N, M = input("Set N and M (example: '9 5': ").split()
    N = int(N)
    M = int(M)

    # Create linked list
    first_node = node
    for x in range(2, N + 1):
        node.next = Node(x)
        node = node.next

    # Make the linked list into a circular linked list
    node.next = first_node

    # Josephus problem
    while node != node.next:
        for _ in range(1, M):
            node = node.next
        print(f"{node.next.key}", end=" ")
        node.next = node.next.next

    print(f"{node.key}")


if __name__ == "__main__":
    main()
        
