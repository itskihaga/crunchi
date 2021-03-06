use std::{cell::{RefCell}, collections::HashSet, hash::Hash, fmt::Display, rc::{Rc}};

use crate::testing::Counter;

pub mod testing {
    use super::*;

    #[derive(Debug,Clone,Default,PartialEq)]
    pub struct Counter {
        num: Rc<RefCell<usize>>,
    }

    impl Counter {
        pub fn count(&self) {
            let cur = *self.num.borrow();
            self.num.replace(cur + 1);
        }
        pub fn get(&self) -> usize {
            *self.num.borrow()
        }
        pub fn map<T, F: FnOnce(Self) -> T>(&self, func: F) -> T {
            func(self.clone())
        }
        pub fn new() -> Self {
            Self::default()
        }
    }

    impl PartialEq<usize> for Counter {
        fn eq(&self, other: &usize) -> bool {
            *self.num.borrow() == *other
        }
    }

    impl Display for Counter {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", *self.num.borrow())
        }
    }

    fn _test_counter() {
        let cnt = Counter::new();
        (0..3)
            .into_iter()
            .map(|_| {
                Box::new(|| {
                    cnt.count();
                }) as Box<dyn FnMut()>
            })
            .for_each(|mut call| call());
        assert_eq!(cnt, 3);
    }

    #[test]
    fn test_counter() {
        _test_counter()
    }
}

type ToOnceListen = Box<dyn FnMut(Box<dyn FnMut()>)-> Box<dyn FnMut()>>;
/**
 * 使ってないけど便利そう
 */
pub fn to_once<L: FnMut(Box<dyn FnMut()>)-> Box<dyn FnMut()> + 'static>(mut listen: L) -> ToOnceListen {
    Box::new(move |mut callback| {
        let shared_unlisten = Rc::new(RefCell::new(Box::new(|| ()) as Box<dyn FnMut()>));
        let cloned_unlisten = shared_unlisten.clone();
        let unlisten = listen(Box::new(move || {
            callback();
            cloned_unlisten.borrow_mut()();
        }));
        drop(shared_unlisten.replace(unlisten));
        let cloned_unlisten = shared_unlisten.clone();
        Box::new(move || cloned_unlisten.borrow_mut()())
    })
}

fn _test_to_once_call_remove_fn() {
    let caller = Rc::new(RefCell::new(Box::new(|| ()) as Box<dyn FnMut()>));
    let cloned = caller.clone();
    let cnt_rm = Counter::new();
    let mut listen_once = cnt_rm.map(|cnt_rm| to_once(move |callback|{
        drop(cloned.replace(callback));
        let cloned_cnt_rm = cnt_rm.clone();
        Box::new(move || cloned_cnt_rm.count())
    }));
    let cnt_called = Counter::new();
    let _ = listen_once(
        cnt_called.map(|cnt| Box::new(move || cnt.count()))
    );
    caller.borrow_mut()();
    assert!(cnt_called == 1);
    assert!(cnt_rm == 1);
}
#[test]
fn test_to_once_call_remove_fn() {
    _test_to_once_call_remove_fn() 
}

#[test]
fn test_call_while_living() {
    _test_call_while_living()
}

fn _test_call_while_living() {
    let counter = Counter::new();
    let mut call = {
        let cnt = counter.map(|counter| Box::new(move || counter.count())  as Box<dyn FnMut()>);
        let rc = Rc::new(RefCell::new(cnt));
        let mut call = call_while_living(&rc, |cnt| cnt.borrow_mut()());
        call();
        assert!(counter == 1);
        call
    };
    call();
    assert!(counter == 1);
}

/**
 * 渡した参照が生存してる場合のみ実行されるCallbackを生成する
 * （使ってない）
 */
pub fn call_while_living<T : 'static,CB: FnMut(&T) + 'static>(target: &Rc<T>,mut call: CB) -> Box<dyn FnMut()> {
    let weak = Rc::downgrade(&target);
    Box::new(move || { weak.upgrade().map(|target| call(&target)); })
}

#[test]
fn test_validate_no_duplicate() {
    assert_eq!(validate_no_duplicate(&vec!["hoge","fuga"],|item| item),true);
    assert_eq!(validate_no_duplicate(&vec!["hoge","fuga","hoge"],|item| item),false);
}
pub fn validate_no_duplicate<'a,I,T : Hash + Eq,F:Fn(&'a I) -> T>(inputs: &'a Vec<I>,get_key: F) -> bool {
    inputs.iter().map(get_key).scan(HashSet::with_capacity(inputs.len()), |state,item| {
        if state.contains(&item) {
            Some(false)
        } else {
            state.insert(item);
            Some(true)
        }
    }).all(|item| item)
}