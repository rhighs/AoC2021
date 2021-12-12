from typing import List
import sys

class Node:
    def __init__(self, name: str, upper: bool) -> 'Node':
        self.name = name
        self.upper = upper

class Graph:
    def __init__(self) -> 'Graph':
        self.edges = {}

def make_graph(lines: List[str]) -> Graph:
    graph = Graph()
    nodes = []
    for line in lines:
        for node in list(line.split("-")):
            nodes.append(node)
    nodes = list(set(nodes))
    for node in nodes:
        graph.edges[node] = []
    for line in lines:
        edge = list(line.split("-"))
        graph.edges[edge[0]].append(Node(edge[1], edge[1].isupper()))
        graph.edges[edge[1]].append(Node(edge[0], edge[0].isupper()))
    return graph

def paths_p1(graph: Graph, current_path: List[str], found_paths: List[List[str]]):
    last = current_path[len(current_path) - 1]
    for node in graph.edges[last]:
        new_paths = [*current_path]
        new_paths.append(node.name)
        if node.name == "end":
            found_paths.append(new_paths)
            continue
        if not node.upper and node.name in current_path:
            continue
        paths_p1(graph, new_paths, found_paths)

def paths_p2(graph: Graph, current_path: List[str], has_double: bool, found_paths: List[List[str]]):
    last = current_path[len(current_path) - 1]
    for node in graph.edges[last]:
        if node.name == "start":
            continue
        new_paths = [*current_path]
        new_paths.append(node.name)
        if node.name == "end":
            found_paths.append(new_paths)
            continue
        if not node.upper and has_double and node.name in current_path:
            continue
        only_lower = [node for node in new_paths if not node.isupper()]
        this_has_double = len(only_lower) != len(set(only_lower))
        paths_p2(graph, new_paths, this_has_double, found_paths)

def main():
    lines: List[str] = []
    with open("./" + str(sys.argv[1]), "r") as file:
        for line in file.readlines():
            line = line.replace("\n", "")
            lines.append(line)

    graph = make_graph(lines)
    current_path = ["start"]
    found_paths = []
    paths_p1(graph, current_path, found_paths)
    print("p1:", len(found_paths))

    graph = make_graph(lines)
    current_path = ["start"]
    found_paths = []
    paths_p2(graph, current_path, False, found_paths)
    print("p2:", len(found_paths))

if __name__ == "__main__":
    main()

