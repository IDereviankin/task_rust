#[derive(PartialEq, Eq, Clone)]
enum Value {
    Number(i32),
    Any,
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Any => write!(f, "`any`"),
        }
    }
}

impl Value {
    fn assume_number(self) -> i32 {
        match self {
            Value::Number(n) => n,
            Value::Any => unreachable!("Assuming value is a number"),
        }
    }
}

fn attempt(available: &[i32], allowed: &[Value], preferred: &[Value]) -> Vec<i32> {
    find_preferred(
        filter_allowed(available.to_vec(), allowed.to_vec()),
        preferred.to_vec(),
    )
}

fn filter_allowed(available: Vec<i32>, allowed: Vec<Value>) -> Vec<i32> {
    if allowed.iter().any(|x| *x == Value::Any) {
        available
    } else {
        let mut result = vec![];
        let mut available = available.into_iter().peekable();
        let mut allowed = allowed.into_iter().map(|x| x.assume_number()).peekable();

        loop {
            match (available.peek(), allowed.peek()) {
                (Some(&av), Some(&al)) => match av.cmp(&al) {
                    std::cmp::Ordering::Greater => {
                        allowed.next();
                    }
                    std::cmp::Ordering::Less => {
                        available.next();
                    }
                    std::cmp::Ordering::Equal => {
                        result.push(av);
                        available.next();
                        allowed.next();
                    }
                },
                _ => break,
            }
        }

        result
    }
}

fn find_preferred(available: Vec<i32>, preferred: Vec<Value>) -> Vec<i32> {
    if preferred.iter().any(|x| *x == Value::Any) {
        available.to_vec()
    } else {
        let mut result = vec![];

        for pref in preferred.into_iter().map(|x| x.assume_number()) {
            let mut index = available.partition_point(|x| *x < pref);
            if index > 0 && available.len() == index {
                index -= 1;
            }
            if available.len() >= 1 {
                result.push(available[index]);
            }
        }

        result.dedup();
        result
    }
}

fn print_attempt(available: &[i32], allowed: &[Value], preferred: &[Value]) {
    print!("available : [ ");
    available.iter().for_each(|x| print!("{:?}, ", x));
    println!("]");
    print!("allowed   : [ ");
    allowed.iter().for_each(|x| print!("{:?}, ", x));
    println!("]");
    print!("preferred : [ ");
    preferred.iter().for_each(|x| print!("{:?}, ", x));
    println!("]");
    let output = attempt(available, allowed, preferred);
    print!("returns   : [ ");
    output.iter().for_each(|x| print!("{:?}, ", x));
    println!("]");
    println!();
}

fn main() {
    use Value::*;

    print_attempt(
        &[240, 360, 720],
        &[Number(360), Number(720)],
        &[Number(1080)],
    );
    print_attempt(&[240, 720], &[Number(360), Number(720)], &[Number(1080)]);
    print_attempt(&[240], &[Number(360), Number(720)], &[Number(1080)]);
    print_attempt(
        &[240, 360, 720],
        &[Number(240), Number(360), Number(720), Number(1080)],
        &[Number(240), Number(360)],
    );
    print_attempt(
        &[240, 720],
        &[Number(240), Number(360), Number(720), Number(1080)],
        &[Number(240), Number(360)],
    );
    print_attempt(
        &[240, 720],
        &[Number(240), Number(360), Number(1080)],
        &[Number(240), Number(360)],
    );
    print_attempt(
        &[720],
        &[Number(240), Number(360), Number(1080)],
        &[Number(240), Number(360)],
    );
    print_attempt(
        &[240, 360],
        &[Number(240), Number(360)],
        &[Number(720), Number(1080)],
    );
    print_attempt(
        &[240, 360, 720],
        &[Number(360), Any],
        &[Number(360), Number(720)],
    );
    print_attempt(
        &[240, 360, 720],
        &[Number(240), Number(360), Number(720)],
        &[Any, Number(720)],
    );
    print_attempt(
        &[240, 360, 720],
        &[Number(360), Number(1080)],
        &[Any, Number(720)],
    );
    print_attempt(&[240, 360, 720], &[Number(1080)], &[Any, Number(720)]);
}

#[cfg(test)]
mod tests {
    use crate::attempt;
    use crate::Value::*;

    #[test]
    fn test() {
        assert_eq!(
            attempt(
                &[240, 360, 720],
                &[Number(360), Number(720)],
                &[Number(1080)],
            ),
            vec![720]
        );
        assert_eq!(
            attempt(&[240, 720], &[Number(360), Number(720)], &[Number(1080)]),
            vec![720]
        );
        assert_eq!(
            attempt(&[240], &[Number(360), Number(720)], &[Number(1080)]),
            vec![]
        );
        assert_eq!(
            attempt(
                &[240, 360, 720],
                &[Number(240), Number(360), Number(720), Number(1080)],
                &[Number(240), Number(360)],
            ),
            vec![240, 360]
        );
        assert_eq!(
            attempt(
                &[240, 720],
                &[Number(240), Number(360), Number(720), Number(1080)],
                &[Number(240), Number(360)],
            ),
            vec![240, 720]
        );
        assert_eq!(
            attempt(
                &[240, 720],
                &[Number(240), Number(360), Number(1080)],
                &[Number(240), Number(360)],
            ),
            vec![240]
        );
        assert_eq!(
            attempt(
                &[720],
                &[Number(240), Number(360), Number(1080)],
                &[Number(240), Number(360)],
            ),
            vec![]
        );
        assert_eq!(
            attempt(
                &[240, 360],
                &[Number(240), Number(360)],
                &[Number(720), Number(1080)],
            ),
            vec![360]
        );

        // `any` tests
        assert_eq!(
            attempt(
                &[240, 360, 720],
                &[Number(360), Any],
                &[Number(360), Number(720)],
            ),
            vec![360, 720]
        );
        assert_eq!(
            attempt(
                &[240, 360, 720],
                &[Number(240), Number(360), Number(720)],
                &[Any, Number(720)],
            ),
            vec![240, 360, 720]
        );
        assert_eq!(
            attempt(
                &[240, 360, 720],
                &[Number(360), Number(1080)],
                &[Any, Number(720)],
            ),
            vec![360]
        );
        assert_eq!(
            attempt(&[240, 360, 720], &[Number(1080)], &[Any, Number(720)],),
            vec![]
        );
    }
}
