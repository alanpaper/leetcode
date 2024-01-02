#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
    out: bool,
}

#[derive(Debug)]
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
    fn insert(&mut self, point: &Point) -> bool {
        if self.data.len() == 0 {
            Link::push(self, point);
            return true;
        } else {
            if Link::o_is_neighbour(self, point) {
                Link::push(self, point);
                return true;
            }
        }
        false
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

fn gentrate_link(point: &Point, links: &mut Vec<Link>) {
    let mut not_insert = false;
    for link_item in links.iter_mut() {
        let result = link_item.insert(point);
        if !not_insert {
            not_insert = result;
        }
    }
    if !not_insert {
        let mut link: Link = Link {
            out: false,
            data: vec![],
        };
        link.insert(point);
        links.push(link);
    }
}

pub fn solve(board: &mut Vec<Vec<char>>) {
    let mut links: Vec<Link> = vec![];
    for (i, row) in board.clone().iter().enumerate() {
        for (y, char) in row.iter().enumerate() {
            if *char == 'O' {
                let point = Point::new(i as i32, y as i32, board.len() as i32, row.len() as i32);
                gentrate_link(&point, &mut links);
            }
        }
    }

    println!("{:?}", links);

    for (i, row) in board.clone().iter().enumerate() {
        for (y, _char) in row.iter().enumerate() {
            for link in links.iter() {
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
