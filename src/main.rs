use std::fmt;

const LEFT_START_POS: (i8, i8) = (3, 0);
const RIGHT_START_POS: (i8, i8) = (3, 2);

#[derive(Copy, Clone)]
enum Choice {
   Left,
   Right
}

impl fmt::Debug for Choice {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
         Choice::Left => write!(f, "Left"),
         Choice::Right => write!(f, "Right"),
      }
   }
}

impl fmt::Display for Choice {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
         Choice::Left => write!(f, "Left"),
         Choice::Right => write!(f, "Right"),
      }
   }
}

struct State {
   left_stack: Vec<(i8, i8)>,
   right_stack: Vec<(i8, i8)>,
   cur_total_distance: f64,
   best_total_distance: f64,
   cur_path_stack: Vec<Choice>,
   best_path_stack: Vec<Choice>,
   nodes_visited: usize,
}

fn main() {
   /*
      1 2 3
      4 5 6
      7 8 9
      * 0 #
   */

  let number_to_dial = [7, 4, 7, 4, 7, 4, 7, 4, 7, 4];
  let mut state = State {
     cur_total_distance: 0.0,
     best_total_distance: std::f64::INFINITY,
     left_stack: Vec::new(),
     right_stack: Vec::new(),
     cur_path_stack: Vec::new(),
     best_path_stack: Vec::new(),
     nodes_visited: 0,
  };

  state.left_stack.push(LEFT_START_POS);
  state.right_stack.push(RIGHT_START_POS);

  dfs(&mut state, &number_to_dial);

  println!("{} after {} nodes visited", state.best_total_distance, state.nodes_visited);
  println!("choices: {:#?}", state.best_path_stack);
}

fn dfs(state: &mut State, number_to_dial: &[i8]) {
   // bound
   if state.cur_total_distance > state.best_total_distance {
      return;
   }

   if state.cur_path_stack.len() == number_to_dial.len() {
      if state.cur_total_distance < state.best_total_distance {
         state.best_total_distance = state.cur_total_distance;
         state.best_path_stack = state.cur_path_stack.clone();
      }
      return;
   }

   state.nodes_visited += 1;

   let target_pos = pos(number_to_dial[state.cur_path_stack.len()]);
   let left_pos = state.left_stack.last().cloned().unwrap();
   let right_pos = state.right_stack.last().cloned().unwrap();
   // move our left finger to this number
   {
      let dist = euclid_distance(left_pos, target_pos);
      state.cur_total_distance += dist;
      state.cur_path_stack.push(Choice::Left);
      state.left_stack.push(target_pos);
      state.right_stack.push(right_pos);
      dfs(state, number_to_dial);
      state.right_stack.pop();
      state.left_stack.pop();
      state.cur_path_stack.pop();
      state.cur_total_distance -= dist;
   }
   // move our right finger to this number
   {
      let dist = euclid_distance(right_pos, target_pos);
      state.cur_total_distance += dist;
      state.cur_path_stack.push(Choice::Right);
      state.left_stack.push(left_pos);
      state.right_stack.push(target_pos);
      dfs(state, number_to_dial);
      state.right_stack.pop();
      state.left_stack.pop();
      state.cur_path_stack.pop();
      state.cur_total_distance -= dist;
   }
}

fn pos(key: i8) -> (i8, i8) {
   match key {
      1 => (0, 0),
      2 => (0, 1),
      3 => (0, 2),
      4 => (1, 0),
      5 => (1, 1),
      6 => (1, 2),
      7 => (2, 1),
      8 => (2, 2),
      9 => (2, 3),
      0 => (3, 1),
      _ => unreachable!()
   }
}

fn euclid_distance(a: (i8, i8), b: (i8, i8)) -> f64 {
   (((a.0 - b.0).pow(2) + (a.1 - b.1).pow(2)) as f64).sqrt()
}
