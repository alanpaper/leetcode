#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
    out: bool,
}

#[derive(Debug, Clone)]
struct Link {
    out: bool,
    data: Vec<Point>,
}

impl Point {
    fn new(x: i32, y: i32, x_len: i32, y_len: i32) -> Point {
        let mut out = false;
        if x == 0 || y == 0 || x == x_len - 1 || y == y_len - 1 {
            out = true;
        }
        Point { x, y, out }
    }
}

impl Link {
    fn reset(&mut self) {
        self.out = false;
        self.data = vec![];
    }

    fn concat(&mut self, link: &Link) {
        for point in &link.data {
            if !self.out && point.out {
                self.out = true;
            }
            Link::push(self, &point);
        }
    }

    // 判断两条线是否相交
    fn intersection(&mut self, link: &Link) -> bool {
        let mut result = false;
        for point in &link.data {
            if Link::o_is_neighbour(self, &point) {
                result = true;
                break;
            }
        }
        result
    }

    fn push(&mut self, point: &Point) {
        if point.out && !self.out {
            self.out = true;
        }
        self.data.push(point.clone());
    }

    // 判断o与o是不是邻居
    fn o_is_neighbour(&self, point: &Point) -> bool {
        let mut result = false;
        for i in self.data.iter() {
            if ((i.x - point.x).abs() == 1 && i.y == point.y)
                || ((i.y - point.y).abs() == 1 && i.x == point.x)
            {
                result = true;
                break;
            }
        }
        result
    }
}

fn intersection_link_concat(link: &Link, links: &mut Vec<Link>) {
    let mut not_insert = false;
    for link_item in links.iter_mut() {
        not_insert = link_item.intersection(link);
        if not_insert {
            link_item.concat(link);
            break;
        }
    }

    if !not_insert && link.data.len() > 0 {
        links.push(link.clone());
    }
}

pub fn solve(board: &mut Vec<Vec<char>>) {
    let mut links: Vec<Link> = vec![];
    // 相邻的o点栈
    let mut o_neighbour_stack: Link = Link {
        out: false,
        data: vec![],
    };

    for (i, row) in board.clone().iter().enumerate() {
        for (y, char) in row.iter().enumerate() {
            if *char == 'O' {
                let point = Point::new(i as i32, y as i32, board.len() as i32, row.len() as i32);
                o_neighbour_stack.push(&point);
            } else {
                intersection_link_concat(&o_neighbour_stack, &mut links);
                o_neighbour_stack.reset();
            }
        }
        if o_neighbour_stack.data.len() > 0 {
            intersection_link_concat(&o_neighbour_stack, &mut links);
            o_neighbour_stack.reset();
        }
    }
    if o_neighbour_stack.data.len() > 0 {
        intersection_link_concat(&o_neighbour_stack, &mut links);
        o_neighbour_stack.reset();
    }

    let mut new_links: Vec<Link> = vec![];
    for link in links {
        intersection_link_concat(&link, &mut new_links);
    }

    for (i, row) in board.clone().iter().enumerate() {
        for (y, _char) in row.iter().enumerate() {
            for link in new_links.iter() {
                if !link.out {
                    for point in link.data.iter() {
                        if point.x == i as i32 && point.y == y as i32 {
                            board[i][y] = 'X';
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test_5() {
        let mut board = vec![
            vec!['X', 'O', 'X', 'O', 'O', 'O', 'O'],
            vec!['X', 'O', 'O', 'O', 'O', 'O', 'O'],
            vec!['X', 'O', 'O', 'O', 'O', 'X', 'O'],
            vec!['O', 'O', 'O', 'O', 'X', 'O', 'X'],
            vec!['O', 'X', 'O', 'O', 'O', 'O', 'O'],
            vec!['O', 'O', 'O', 'O', 'O', 'O', 'O'],
            vec!['O', 'X', 'O', 'O', 'O', 'O', 'O'],
        ];
        solve(&mut board);
        let resultvalue = vec![
            vec!['X', 'O', 'X', 'O', 'O', 'O', 'O'],
            vec!['X', 'O', 'O', 'O', 'O', 'O', 'O'],
            vec!['X', 'O', 'O', 'O', 'O', 'X', 'O'],
            vec!['O', 'O', 'O', 'O', 'X', 'O', 'X'],
            vec!['O', 'X', 'O', 'O', 'O', 'O', 'O'],
            vec!['O', 'O', 'O', 'O', 'O', 'O', 'O'],
            vec!['O', 'X', 'O', 'O', 'O', 'O', 'O'],
        ];
        assert_eq!(board, resultvalue)
    }

    #[test]
    fn solve_test() {
        let mut board = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        solve(&mut board);
        let resultvalue = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        assert_eq!(board, resultvalue)
    }

    #[test]
    fn solve_test_49() {
        let mut board = vec![
            vec!['O', 'X', 'O', 'O', 'X', 'X'],
            vec!['O', 'X', 'X', 'X', 'O', 'X'],
            vec!['X', 'O', 'O', 'X', 'O', 'O'],
            vec!['X', 'O', 'X', 'X', 'X', 'X'],
            vec!['O', 'O', 'X', 'O', 'X', 'X'],
            vec!['X', 'X', 'O', 'O', 'O', 'O'],
        ];

        solve(&mut board);
        let resultvalue = vec![
            vec!['O', 'X', 'O', 'O', 'X', 'X'],
            vec!['O', 'X', 'X', 'X', 'O', 'X'],
            vec!['X', 'O', 'O', 'X', 'O', 'O'],
            vec!['X', 'O', 'X', 'X', 'X', 'X'],
            vec!['O', 'O', 'X', 'O', 'X', 'X'],
            vec!['X', 'X', 'O', 'O', 'O', 'O'],
        ];
        assert_eq!(board, resultvalue)
    }

    #[test]
    fn solve_test_two() {
        let mut board = vec![
            vec!['O', 'X', 'X', 'O', 'X'],
            vec!['X', 'O', 'O', 'X', 'O'],
            vec!['X', 'O', 'X', 'O', 'X'],
            vec!['O', 'X', 'O', 'O', 'O'],
            vec!['X', 'X', 'O', 'X', 'O'],
        ];
        solve(&mut board);
        let resultvalue = vec![
            vec!['O', 'X', 'X', 'O', 'X'],
            vec!['X', 'X', 'X', 'X', 'O'],
            vec!['X', 'X', 'X', 'O', 'X'],
            vec!['O', 'X', 'O', 'O', 'O'],
            vec!['X', 'X', 'O', 'X', 'O'],
        ];
        assert_eq!(board, resultvalue)
    }

    #[test]
    fn solve_test_four() {
        let mut board = vec![
            vec!['O', 'O', 'O', 'O', 'X', 'X'],
            vec!['O', 'O', 'O', 'O', 'O', 'O'],
            vec!['O', 'X', 'O', 'X', 'O', 'O'],
            vec!['O', 'X', 'O', 'O', 'X', 'O'],
            vec!['O', 'X', 'O', 'X', 'O', 'O'],
            vec!['O', 'X', 'O', 'O', 'O', 'O'],
        ];
        solve(&mut board);
        let resultvalue = vec![
            vec!['O', 'O', 'O', 'O', 'X', 'X'],
            vec!['O', 'O', 'O', 'O', 'O', 'O'],
            vec!['O', 'X', 'O', 'X', 'O', 'O'],
            vec!['O', 'X', 'O', 'O', 'X', 'O'],
            vec!['O', 'X', 'O', 'X', 'O', 'O'],
            vec!['O', 'X', 'O', 'O', 'O', 'O'],
        ];
        assert_eq!(board, resultvalue)
    }

    #[test]
    fn solve_test_three() {
        let mut board = vec![
            vec!['X', 'O', 'O', 'X', 'X', 'X', 'O', 'X', 'O', 'O'],
            vec!['X', 'O', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X', 'O', 'X', 'X', 'X', 'X', 'X'],
            vec!['X', 'O', 'X', 'X', 'X', 'O', 'X', 'X', 'X', 'O'],
            vec!['O', 'X', 'X', 'X', 'O', 'X', 'O', 'X', 'O', 'X'],
            vec!['X', 'X', 'O', 'X', 'X', 'O', 'O', 'X', 'X', 'X'],
            vec!['O', 'X', 'X', 'O', 'O', 'X', 'O', 'X', 'X', 'O'],
            vec!['O', 'X', 'X', 'X', 'X', 'X', 'O', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X', 'X', 'O', 'X', 'X', 'O', 'O'],
            vec!['X', 'X', 'X', 'O', 'O', 'X', 'O', 'X', 'X', 'O'],
        ];
        solve(&mut board);
        let resultvalue = vec![
            vec!['X', 'O', 'O', 'X', 'X', 'X', 'O', 'X', 'O', 'O'],
            vec!['X', 'O', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'O'],
            vec!['O', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X'],
            vec!['O', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'O'],
            vec!['O', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'O', 'O'],
            vec!['X', 'X', 'X', 'O', 'O', 'X', 'O', 'X', 'X', 'O'],
        ];
        assert_eq!(board, resultvalue)
    }
}
