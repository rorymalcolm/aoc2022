import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;

public class DaySeven {
  private static final Integer PART_ONE_SIZE = 100_000;

  private static final Integer PART_TWO_SIZE = 30000000;

  private static final Integer TOTAL_FILESYSTEM_SIZE = 70000000;
  public static void main(String[] args) throws IOException {
    final List<String> fileLines = Files.lines(Path.of("src/main/resources/seven.txt")).toList();
    final Directory root = new Directory("/");
    processLine(root, fileLines);
    root.prettyPrintTree();
    final var directories = Directory.findSumOfDirectoryWithSizeUnder(root, PART_ONE_SIZE);
    System.out.println("Part1: " + directories.stream().mapToLong(DirectoryNameAndSize::size).sum());
    final var dirs = Directory.collectAllDirectories(root);
    final var TOTAL_SIZE = Directory.totalSizeOfChildren(root.childNodes, root.childDirectories);
    final var UNUSED_SPACE = TOTAL_FILESYSTEM_SIZE - TOTAL_SIZE;
    final var TOTAL_REQUIRED = PART_TWO_SIZE - UNUSED_SPACE;
    final var REMOVED_DIR_SIZE = dirs.stream().map(dir -> Directory.totalSizeOfChildren(dir.childNodes, dir.childDirectories)).filter(size -> size > TOTAL_REQUIRED).min(Long::compareTo).orElseThrow();
    System.out.println("Part2: " + REMOVED_DIR_SIZE);
  }

  public static void processLine(Directory current, List<String> lines) {
    processLine(current, current, lines, 0);
  }

  private static void processLine(Directory root, Directory current, List<String> lines, int depth) {
    if (lines.isEmpty() || depth > lines.size() - 1) {
      return;
    }
    final String line = lines.get(depth);
    if (line.isBlank()) {
      return;
    }
    if (line.startsWith("$")) {
      if (line.contains("cd")) {
        if (line.equals("$ cd ..")) {
          if (current.parent.isPresent()) {
            processLine(root, current.parent.get(), lines, depth + 1);
          } else {
            processLine(root, root, lines, depth + 1);
          }
          return;
        } else if (line.equals("$ cd /")) {
          processLine(root, root, lines, depth + 1);
          return;
        } else {
          final String directoryName = line.split(" ")[2];
          final Directory directory = new Directory(directoryName, current);
          current.insertDirectory(directory);
          processLine(root, directory, lines, depth + 1);
          return;
        }
      }
      processLine(root, current, lines, depth + 1);
    } else if (!line.startsWith("dir")) {
      final List<String> split = List.of(line.split(" "));
      current.insertChildNode(new Node(current, split.get(1), Integer.parseInt(split.get(0))));
      processLine(root, current, lines, depth + 1);
    } else {
      processLine(root, current, lines, depth + 1);
    }
  }
}
