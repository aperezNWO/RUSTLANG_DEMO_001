use rand::Rng;

pub fn run_random_dijkstra() -> String {
    let (vertex_size, sample_size, source_point) = (9, 23, 0);
    let sample_size_adj = sample_size - 2;

    let mut vertex_x = (1..=sample_size_adj).collect::<Vec<i32>>();
    let mut vertex_y = (1..=sample_size_adj).collect::<Vec<i32>>();

    fisher_yates(&mut vertex_x);
    fisher_yates(&mut vertex_y);

    let mut vertex_array = Vec::new();
    for i in 0..vertex_size {
        let sep = if i < vertex_size - 1 { "|" } else { "" };
        vertex_array.push(format!("[{},{}]{}", vertex_x[i], vertex_y[i], sep));
    }

    let vertex_array_string = vertex_array.join("");
    let mut graph = vec![vec![0i32; vertex_size]; vertex_size];

    let vertex_matrix = generate_random_matrix(&vertex_array, &mut graph, vertex_size);
    let vertex_list = dijkstra_core(&vertex_array, &graph, vertex_size, source_point);

    let sorted_list_encoded = vertex_list.replace(",", "<br/>").replace("\t", "&nbsp;");
    format!("{}■{}■{}", vertex_array_string, vertex_matrix, sorted_list_encoded)
}

fn fisher_yates(deck: &mut [i32]) {
    let mut rng = rand::thread_rng();
    for i in (1..deck.len()).rev() {
        let j = rng.gen_range(0..=i);
        deck.swap(i, j);
    }
}

fn generate_random_matrix(vertex_string: &[String], graph: &mut [Vec<i32>], vertex_size: usize) -> String {
    let mut rng = rand::thread_rng();

    for x in 0..vertex_size {
        for y in (x + 1)..vertex_size {
            let val = if rng.gen_bool(0.5) { get_hypotenuse(vertex_string, x, y) as i32 } else { 0 };
            graph[x][y] = val;
            graph[y][x] = val;
        }
    }

    for x in 0..vertex_size {
        let mut zero_count = 0;
        for y in 0..vertex_size {
            if x != y && graph[x][y] == 0 {
                zero_count += 1;
                if zero_count == vertex_size - 1 {
                    let hyp = get_hypotenuse(vertex_string, x, y) as i32;
                    graph[x][y] = hyp;
                    graph[y][x] = hyp;
                }
            }
        }
    }

    graph.iter()
        .map(|row| format!("{{{}}}", row.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(",")))
        .collect::<Vec<_>>()
        .join("|")
}

fn get_hypotenuse(vertex_string: &[String], index_x: usize, index_y: usize) -> f64 {
    let parse_coord = |s: &String| -> (f64, f64) {
        let clean = s.replace(['|', '[', ']'], "");
        let parts: Vec<&str> = clean.split(',').collect();
        (parts[0].parse().unwrap_or(0.0), parts[1].parse().unwrap_or(0.0))
    };

    let (src_x, src_y) = parse_coord(&vertex_string[index_y]);
    let (dest_x, dest_y) = parse_coord(&vertex_string[index_x]);

    ((dest_x - src_x).powi(2) + (dest_y - src_y).powi(2)).sqrt()
}

fn dijkstra_core(vertex: &[String], graph: &[Vec<i32>], vertex_size: usize, src: usize) -> String {
    let mut dist = vec![i32::MAX; vertex_size];
    let mut visited = vec![false; vertex_size];
    let mut previous = vec![None; vertex_size];

    dist[src] = 0;

    for _ in 0..vertex_size {
        let mut min_dist = i32::MAX;
        let mut u = None;

        for i in 0..vertex_size {
            if !visited[i] && dist[i] < min_dist {
                min_dist = dist[i];
                u = Some(i);
            }
        }

        let u = match u {
            Some(val) => val,
            None => break,
        };

        visited[u] = true;

        for v in 0..vertex_size {
            let weight = graph[u][v];
            if !visited[v] && weight > 0 && dist[u] != i32::MAX {
                let new_dist = dist[u] + weight;
                if new_dist < dist[v] {
                    dist[v] = new_dist;
                    previous[v] = Some(u);
                }
            }
        }
    }

    let mut result = Vec::new();
    for v in 0..vertex_size {
        let mut d = dist[v];
        if d == i32::MAX { d = 0; }

        let mut path_str = String::new();
        if v != src && dist[v] != i32::MAX {
            let mut steps = Vec::new();
            let mut curr = Some(v);
            while let Some(c) = curr {
                steps.push(c);
                if c == src { break; }
                curr = previous[c];
            }
            steps.reverse();

            if !steps.is_empty() && steps[0] == src {
                for i in 0..steps.len() - 1 {
                    path_str.push_str(&format!("[{};{}]≡", steps[i], steps[i + 1]));
                }
            }
        }

        let v_clean = vertex[v].replace(',', ";").replace('|', "");
        result.push(format!("{:02}<{}>-{:02}-{}", v, v_clean, d, path_str));
    }

    result.join(",")
}
