use std::time::{Duration, Instant};
use std::rc::{Rc, Weak};
use std::cell::RefCell;


thread_local! {
    static STACK: RefCell<Stack> = RefCell::new(Stack::new());
}

static mut ENABLED: bool = false;

/*static STACK_LIST: Lazy<RwLock<StackList>> = Lazy::new(|| RwLock::new(StackList {
    stacks: Vec::new()
}));


struct StackList {
    stacks: Vec<(Thread, Rc<RefCell<Frame>>)>
}*/


struct Stack {
    thread_name: String,
    root_frame: Rc<RefCell<Frame>>,
    current_frame: Rc<RefCell<Frame>>,
}

/*impl Drop for Stack {
    fn drop(&mut self) {
        let current_thread = std::thread::current();
        STACK_LIST.write().unwrap().stacks.retain(move |(thread, _)| current_thread.id() == thread.id());
    }
}*/

impl Stack {

    fn new() -> Self {

        let thread = std::thread::current();
        let root_frame = Frame::new("root", Weak::new());

        /*STACK_LIST.write().unwrap().stacks.push((
            thread.clone(),
            Rc::clone(&root_frame)
        ));*/

        Self {
            thread_name: thread.name().unwrap_or("").to_string(),
            root_frame: Rc::clone(&root_frame),
            current_frame: root_frame
        }

    }

    fn push(&mut self, name: &'static str) {
        let frame = (*self.current_frame).borrow_mut().get_or_create_child(name, &self.current_frame);
        (*frame).borrow_mut().start = Instant::now();
        self.current_frame = frame;
    }

    fn pop(&mut self) {

        let parent_frame = {
            let mut current_frame = (*self.current_frame).borrow_mut();
            let duration = current_frame.start.elapsed();
            current_frame.duration_sum += duration;
            current_frame.duration_count += 1;
            Weak::upgrade(&current_frame.parent)
        };

        self.current_frame = parent_frame.expect("You can't pop the root frame.");

    }

    fn debug(&self) {
        println!("----< thread: {} >----", self.thread_name);
        Self::debug_frame(&self.root_frame, 0);
    }

    fn debug_frame(frame: &Rc<RefCell<Frame>>, tab: u16) {

        let guard = (**frame).borrow();
        for _ in 0..tab {
            print!("  ");
        }

        if guard.duration_count > 0 {
            println!("- {} (x{}, total: {:.2?}, avg: {:.2?})", guard.name, guard.duration_count, guard.duration_sum, guard.duration_sum / guard.duration_count);
        } else {
            println!("- {}", guard.name);
        }

        for child in &guard.children {
            Self::debug_frame(&child.frame, tab + 1);
        }

    }

}


struct Frame {
    /// An identifier name for the frame.
    name: &'static str,
    /// Children frames.
    children: Vec<FrameChild>,
    /// Parent frame, dangling in case of root frame.
    parent: Weak<RefCell<Frame>>,
    /// Last start instant.
    start: Instant,
    /// Total sum of all durations.
    duration_sum: Duration,
    /// The number of samples summed in the `duration_sum` field, used for average.
    duration_count: u32
}

struct FrameChild {
    name: &'static str,
    frame: Rc<RefCell<Frame>>
}

impl Frame {

    fn new(name: &'static str, parent: Weak<RefCell<Frame>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            name,
            children: Vec::new(),
            parent,
            start: Instant::now(),
            duration_sum: Duration::from_secs(0),
            duration_count: 0
        }))
    }

    fn push_child(&mut self, name: &'static str, parent: &Rc<RefCell<Frame>>) -> Rc<RefCell<Frame>> {
        let child = Self::new(name, Rc::downgrade(parent));
        self.children.push(FrameChild {
            name,
            frame: Rc::clone(&child)
        });
        child
    }

    fn get_or_create_child(&mut self, name: &'static str, parent: &Rc<RefCell<Frame>>) -> Rc<RefCell<Frame>> {

        // The algorithm is spacial here because we want to optimize in case of loop on
        // the same child frame. Or if the whole section loop over to the first child.

        let len = self.children.len();

        let end = if len == 0 {
            return self.push_child(name, parent);
        } else if len > 1 {
            let last_child = &self.children[len - 1];
            if last_child.name == name {
                return Rc::clone(&last_child.frame);
            }
            len - 1
        } else {
            len
        };

        // Since the last child has already been checked, ignore it.
        let first_child = self.children[..end].iter()
            .find(move |frame| frame.name == name);

        if let Some(child) = first_child {
            Rc::clone(&child.frame)
        } else {
            self.push_child(name, parent)
        }

    }

}


pub fn push(name: &'static str) {
    if is_enabled() {
        STACK.with(move |stack| stack.borrow_mut().push(name));
    }
}

pub fn pop() {
    if is_enabled() {
        STACK.with(move |stack| stack.borrow_mut().pop());
    }
}

pub fn pop_push(name: &'static str) {
    if is_enabled() {
        STACK.with(move |stack| {
            let mut guard = stack.borrow_mut();
            guard.pop();
            guard.push(name);
        });
    }
}

pub fn frame<F: FnOnce()>(name: &'static str, func: F) {
    if is_enabled() {
        STACK.with(move |stack| {
            let mut stack = stack.borrow_mut();
            stack.push(name);
            func();
            stack.pop();
        });
    }
}

/// Debug the current thread' stack.
pub fn debug() {
    STACK.with(|stack| {
        stack.borrow().debug();
    });
}

#[inline]
fn is_enabled() -> bool {
    unsafe { ENABLED }
}

/// Enable performance profiling, this function is unsafe for now because you need to call
/// it once before any profiling.
#[inline]
pub unsafe fn enable() {
    ENABLED = true;
}

/// Disable performance profiling, this function is unsafe for now because you need to call
/// it once before any profiling.
#[inline]
pub unsafe fn disable() {
    ENABLED = false;
}
