use std::cell::RefCell;
use std::collections::RingBuf;
use std::rc::Rc;
use std::cmp::min;

pub struct FixedBuffer1<A, It> {
    buff: RingBuf<A>,
    capacity: uint,
    input: It,
}
impl<A, It: Iterator<A>> Iterator<A> for FixedBuffer1<A, It> {
    fn next(&mut self) -> Option<A> {
        if self.buff.is_empty() {
            for _ in range(0u, self.capacity) {
                match self.input.next() {
                    Some(a) => self.buff.push_front(a),
                    None => break
                }
            }
        }
        self.buff.pop_back()
    }
}
pub fn buffer_fixed<A, It: Iterator<A>>(it: It, capacity: uint) -> FixedBuffer1<A, It> {
    FixedBuffer1 {
        buff: RingBuf::with_capacity(capacity),
        capacity: capacity,
        input: it
    }
}

struct FixedBuffer2Inner<A, B, It> {
    iter: It,
    first: RingBuf<A>,
    first_capacity: uint,
    second: RingBuf<B>,
    second_capacity: uint,
}
type FixedBuffer2Shared<A, B, It> = Rc<RefCell<FixedBuffer2Inner<A, B, It>>>;

pub struct FixedBuffer2First<A, B, It> {
    data: FixedBuffer2Shared<A, B, It>
}
impl<A,B, It: Iterator<(A,B)>> Iterator<A> for FixedBuffer2First<A, B, It> {
    fn next(&mut self) -> Option<A> {
        let mut inner = self.data.borrow_mut();

        if inner.first.is_empty() {
            let num_to_take: uint = min(inner.first_capacity - inner.first.len(),
                                        inner.second_capacity - inner.second.len());
            if num_to_take == 0 {panic!("Buffer error");}
            for _ in range(0, num_to_take) {
                match inner.iter.next() {
                    Some((a,b)) => {inner.first.push_back(a);
                                    inner.second.push_back(b);},
                    None => break
                }
            }
        }

        inner.first.pop_front()
    }
}

pub struct FixedBuffer2Second<A, B, It> {
    data: FixedBuffer2Shared<A, B, It>
}
impl<A,B, It: Iterator<(A,B)>> Iterator<B> for FixedBuffer2Second<A,B,It> {
    fn next(&mut self) -> Option<B> {
        let mut inner = self.data.borrow_mut();

        if inner.second.is_empty() {
            let num_to_take: uint = min(inner.first_capacity - inner.first.len(),
                                        inner.second_capacity - inner.second.len());
            if num_to_take == 0 {panic!("Buffer error");}
            for _ in range(0, num_to_take) {
                match inner.iter.next() {
                    Some((a,b)) => {inner.first.push_back(a);
                                    inner.second.push_back(b);},
                    None => break
                }
            }
        }

        inner.second.pop_front()
    }
}

pub fn split_fixed<A, B, It: Iterator<(A,B)>>(it: It, cap_a: uint, cap_b: uint) -> 
                                                (FixedBuffer2First<A, B, It>, 
                                                 FixedBuffer2Second<A, B, It>) {
    let data = Rc::new(RefCell::new(FixedBuffer2Inner { 
        iter: it,
        first: RingBuf::with_capacity(cap_a),
        first_capacity: cap_a,
        second: RingBuf::with_capacity(cap_b),
        second_capacity: cap_b,
    }));

    (FixedBuffer2First { data: data.clone() }, FixedBuffer2Second { data: data })
}
