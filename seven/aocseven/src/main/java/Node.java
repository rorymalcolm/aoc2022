public class Node {

  Directory parent;

  public Node(Directory parent, String name, Integer size) {
    this.parent = parent;
    this.name = name;
    this.size = size;
  }

  String name;
  Integer size;
}
