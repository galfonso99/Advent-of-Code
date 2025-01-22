package aoc

import "core:fmt"
import "core:strings"
import "core:testing"
import "core:container/priority_queue"
import "core:slice"

main :: proc() {
    input :: #load("../../input/day16.txt", string)
    //sample :: #load("../../input/test.txt", string)
    //sample :: #load("../../input/test2.txt", string)

    //fmt.printfln("Part 1: %d", part1(sample))
    fmt.printfln("Part 2: %d", part2(input))
}

part1 :: proc(input: string) -> int {
    ta := context.temp_allocator
    lines := strings.split(input, "\n", ta)
    w := len(lines[0])
    h := len(lines) - 1 // trailing newline
	count := 0
    
    walls := make([][]bool, w)
    for x in 0..< w {
        walls[x] = make([]bool, h)
    }
    defer {
        for wall in walls do delete(wall)
        delete(walls)
    }
    
    start_x, start_y, end_x, end_y := 0, 0, 0, 0
    
    for x in 0 ..< w do for y in 0 ..< h {
        if lines[y][x] == '#' do walls[x][y] = true
        if lines[y][x] == 'S' { start_x, start_y = x, y }
        if lines[y][x] == 'E' { end_x, end_y = x, y }
    }
    
	points :=  dijkstra(walls, start_x, start_y, end_x, end_y, w, h, &count)
	return count
}

State :: struct {
    x, y: int,
    dir: int,  // 0 = up, 1 = east, 2 = down, 3 = west
    cost: int,
}

state_priority :: proc(a, b: State) -> bool {
    return a.cost < b.cost
}

dijkstra :: proc(walls: [][]bool, start_x, start_y, end_x, end_y, width, height: int, count: ^int) -> int {
    dirs := [4][2]int{{0, -1}, {1, 0}, {0, 1}, {-1, 0}}
    
    // visited[x][y][direction]
    visited := make([][][]bool, width)
    for x in 0..< width {
        visited[x] = make([][]bool, height)
        for y in 0..< height {
            visited[x][y] = make([]bool, 4)
        }
    }
    defer {
        for x in 0..< width {
            for y in 0..< height {
                delete(visited[x][y])
            }
            delete(visited[x])
        }
        delete(visited)
    }
    
    pq: priority_queue.Priority_Queue(State)
    priority_queue.init(&pq, state_priority, proc(q: []State, i, j: int) { q[i], q[j] = q[j], q[i] })
    defer priority_queue.destroy(&pq)
    
    // Start facing east (direction 1)
    start := State{start_x, start_y, 1, 0}
    priority_queue.push(&pq, start)
    
    for priority_queue.len(pq) > 0 {
        current := priority_queue.pop(&pq)
		count^ += 1
        
        if current.x == end_x && current.y == end_y {
            return current.cost
        }
        
        if visited[current.x][current.y][current.dir] {
            continue
        }
        visited[current.x][current.y][current.dir] = true
        
        for i in 0..<4 {
            rot_cost := 0
            if i == 1 || i == 3 do rot_cost = 1000  // clockwise or counterclockwise
            if i == 2 do rot_cost = 2000            // reverse direction
            
            new_dir := (current.dir + i) % 4
            next_x := current.x + dirs[new_dir][0]
            next_y := current.y + dirs[new_dir][1]
            
            if next_x < 0 || next_x >= width || next_y < 0 || next_y >= height { continue }
            
            if walls[next_x][next_y] || visited[next_x][next_y][new_dir] { continue }
            
            next := State{next_x, next_y, new_dir, current.cost + 1 + rot_cost}
            priority_queue.push(&pq, next)
        }
    }
    return -1
}

// For part2 here is the idea use Dijkstra's to find the shortest path
// Then run dijkstras again without stopping until the resulting points is higher than
// the shortest path points
// Meanwhile while dijkstras is running record the positions of the path
// Then loop through and count the spots with 'O'
// I might need to keep a path array then if a path is one of the best paths go through the array
// and mark the positions with O
part2 :: proc(input: string) -> int {
    ta := context.temp_allocator
    lines := strings.split(input, "\n", ta)
    w := len(lines[0])
    h := len(lines) - 1 // trailing newline
	count := 0
    
    maze := make([][]u8, w)
    for x in 0..< w {
        maze[x] = make([]u8, h)
    }
    defer {
        for row in maze do delete(row)
        delete(maze)
    }
    
    start_x, start_y, end_x, end_y := 0, 0, 0, 0
    
    for x in 0 ..< w do for y in 0 ..< h {
        if lines[y][x] == 'S' { start_x, start_y = x, y; maze[x][y] = '.' }
        if lines[y][x] == 'E' { end_x, end_y = x, y; maze[x][y] = '.' }
        if lines[y][x] == '#' do maze[y][x] = '#'
        if lines[y][x] == '.' do maze[y][x] = '.'
    }
    
	best_path_tiles := 0
	points, steps :=  dijkstra2(&maze, start_x, start_y, end_x, end_y, w, h)

	//for y in 0 ..< h  {
	//	for x in 0 ..< w {
	//		fmt.printf("%c", maze[y][x])
	//	}
	//	fmt.println()
	//}
	for y in 0 ..< h  {
		for x in 0 ..< w {
			if maze[y][x] == 'O' {
				best_path_tiles += 1
			}
		}
	}
	return best_path_tiles
}

State2 :: struct {
    x, y: int,
    dir: int,  // 0 = up, 1 = east, 2 = down, 3 = west
    cost: int,
	steps: int,
	prev: ^State2,
}

dijkstra2 :: proc(maze: ^[][]u8, start_x, start_y, end_x, end_y, width, height: int) -> (int, int) {
    dirs := [4][2]int{{0, -1}, {1, 0}, {0, 1}, {-1, 0}}
    
    // visited[x][y][direction]
    visited := make([][][]bool, width)
    for x in 0..< width {
        visited[x] = make([][]bool, height)
        for y in 0..< height {
            visited[x][y] = make([]bool, 4)
        }
    }
    defer {
        for x in 0..< width {
            for y in 0..< height {
                delete(visited[x][y])
            }
            delete(visited[x])
        }
        delete(visited)
    }
    
	shortest_path := max(int)
    pq: priority_queue.Priority_Queue(State2)
    priority_queue.init(&pq, proc(a,b: State2) -> bool {return a.cost < b.cost}, proc(q: []State2, i, j: int) { q[i], q[j] = q[j], q[i] })
    defer priority_queue.destroy(&pq)
   
    // Start facing east (direction 1)
    start := State2{start_x, start_y, 1, 0, 1, nil}
    priority_queue.push(&pq, start)
    
    for priority_queue.len(pq) > 0 {
        current := priority_queue.pop(&pq)
		// If we reached the end for the second or more time and the cost is higher than
		// 'shortest_path' then simply return
		if current.x == end_x && current.y == end_y && shortest_path < max(int) && current.cost > shortest_path {
			return current.cost, current.steps
		}

        if current.x == end_x && current.y == end_y {
			shortest_path = current.cost
			curr := &current
			for curr != nil {
				maze[curr^.x][curr^.y] = 'O'
				curr = curr.prev
			}
			continue
        }

		if shortest_path < max(int) && current.cost > shortest_path {
			continue
		}
        visited[current.x][current.y][current.dir] = true
        
        for i in 0..<4 {
            rot_cost := 0
            if i == 1 || i == 3 do rot_cost = 1000  // clockwise or counterclockwise
            if i == 2 do rot_cost = 2000            // reverse direction
            
            new_dir := (current.dir + i) % 4
            next_x := current.x + dirs[new_dir][0]
            next_y := current.y + dirs[new_dir][1]

            if maze[next_x][next_y] == '#' || visited[next_x][next_y][new_dir] { continue }
            
			curr := new_clone(current)
            next := State2{next_x, next_y, new_dir, current.cost + 1 + rot_cost, current.steps + 1, curr}
			
            priority_queue.push(&pq, next)
        }
    }
    return -1, -1
}

@(test)
test_day_6 :: proc(t: ^testing.T) {
    sample :: #load("../../input/test.txt", string)

    testing.expect_value(t, part1(sample), 7036)
    //testing.expect_value(t, part2(sample), 6)
}
