import java.util.ArrayList;
import java.util.List;
import java.util.Optional;

public class Directory {

  String name;
  List<Node> childNodes;
  List<Directory> childDirectories;

  @Override
  public String toString() {
    return "Directory{" +
        "name='" + name + '\'' +
        ", childNodes=" + childNodes.size() +
        ", childDirectories=" + childDirectories.size() +
        '}';
  }

  Optional<Directory> parent;

  public Directory(String name) {
    this.name = name;
    this.parent = Optional.empty();
    this.childDirectories = new ArrayList<>();
    this.childNodes = new ArrayList<>();
  }

  public Directory(String name, Directory parent) {
    this.name = name;
    this.parent = Optional.of(parent);
    this.childDirectories = new ArrayList<>();
    this.childNodes = new ArrayList<>();
  }

  public void insertDirectory(Directory directory) {
    childDirectories.add(directory);
  }

  public void insertChildNode(Node node) {
    childNodes.add(node);
  }

  public void prettyPrintTree() {
    prettyPrintTree(this.name, this.childNodes, this.childDirectories, 0);
  }

  private void prettyPrintTree(
      String name, List<Node> childNodes, List<Directory> childDirectories, int level) {
    String indent = " ".repeat(level * 2);
    System.out.println(indent + name + " : " + totalSizeOfChildren(childNodes, childDirectories));
    childNodes.forEach(
        node -> System.out.println(indent + "  " + node.name + " (" + node.size + ")"));
    childDirectories.forEach(
        directory ->
            prettyPrintTree(
                directory.name, directory.childNodes, directory.childDirectories, level + 1));
  }

  public static long totalSizeOfChildren(List<Node> childNodes, List<Directory> childDirectories) {
    return
        childNodes
            .stream()
            .mapToLong(node -> node.size)
            .sum()
            + childDirectories
            .stream()
            .mapToLong(directory -> Directory.totalSizeOfChildren(directory.childNodes,
                directory.childDirectories))
            .sum();
  }

  public static List<DirectoryNameAndSize> findSumOfDirectoryWithSizeUnder(Directory directory,
      int size) {
    List<DirectoryNameAndSize> directories = new ArrayList<>();
    findDirectoryWithSizeUnder(directories, directory, size);
    return directories;
  }

  public static Integer findDirectoryWithSizeUnder(
      List<DirectoryNameAndSize> directories, Directory directory, int size) {
    List<Integer> sizes = new ArrayList<>();
    sizes.add(directory.childNodes.stream().mapToInt(node -> node.size).sum());
    directory.childDirectories.forEach(
        childDirectory -> sizes.add(findDirectoryWithSizeUnder(directories, childDirectory, size)));
    if (sizes.stream().mapToInt(Integer::intValue).sum() < size) {
      directories.add(new DirectoryNameAndSize(directory.name,
          sizes.stream().mapToInt(Integer::intValue).sum()));
    }
    return sizes.stream().mapToInt(Integer::intValue).sum();
  }

  public static List<Directory> collectAllDirectories(Directory directory) {
    List<Directory> directories = new ArrayList<>();
    collectAllDirectories(directories, directory);
    return directories;
  }

  public static List<Directory> collectAllDirectories(List<Directory> directories, Directory
directory) {
    directories.add(directory);
    directory.childDirectories.forEach(childDirectory -> collectAllDirectories(directories,
childDirectory));
    return directories;
  }
}
