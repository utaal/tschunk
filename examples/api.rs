
pub struct TVec<T> {
    len: usize,
    __t: *const T,
}

struct DD {
    pub e: u32,
}

struct CC {
    __d: TVec<DD>,
}

struct BB {
    pub q: u32,
    __c: TVec<CC>,
}

struct AA {
    pub a: u32,
    __b: TVec<BB>,
    __d: TVec<DD>,
}

pub struct BufferTVec<'b, T: OpenTschunk> {
    builder: &'b mut BufferTVecBuilder<T>,
    items: Vec<T>,
}

impl<'b, T: OpenTschunk> BufferTVec<'b, T> {
    fn start_item(&mut self) -> T::Open {
        Default::default()
    }

    fn stop_vec<R>(self, root: &mut Root<R>) {
    }
}

impl<'b, T: OpenTschunk> Drop for BufferTVec<'b, T> {
    fn drop(&mut self) {
        panic!("don't drop incomplete vecs!");
    }
}

pub trait Tschunk where Self: Sized {
    type TVecBuilder;
}

pub struct BufferTVecBuilder<T: OpenTschunk> {
    items: Option<Vec<T>>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: OpenTschunk> Default for BufferTVecBuilder<T> {
    fn default() -> Self {
        BufferTVecBuilder {
            items: None,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: OpenTschunk> BufferTVecBuilder<T> {
    fn start_vec<'b>(&'b mut self) -> BufferTVec<'b, T> {
        BufferTVec {
            builder: self,
            items: Vec::new(),
        }
    }
}

pub struct LeafTVec<'a, 'b, T, R> {
    root: &'a mut Root<R>,
    builder: &'b mut LeafTVecBuilder<T>,
}

impl<'a, 'b, T, R> LeafTVec<'a, 'b, T, R> {
    fn start_item(&mut self) -> &mut T {
        unimplemented!()
    }
    fn done(self) {

    }
}

pub struct LeafTVecBuilder<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Default for LeafTVecBuilder<T> {
    fn default() -> Self {
        LeafTVecBuilder {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T> LeafTVecBuilder<T> {
    fn start_vec<'a, 'b, R>(&'b mut self, root: &'a mut Root<R>) -> LeafTVec<'a, 'b, T, R> where 'a: 'b {
        LeafTVec {
            root,
            builder: self,
        }
    }
}

pub trait OpenTschunk {
    type Open: Default;
}

impl Tschunk for AA {
    type TVecBuilder = BufferTVecBuilder<Self>;
}
impl OpenTschunk for AA {
    type Open = OpenAA;
}
impl Tschunk for BB {
    type TVecBuilder = BufferTVecBuilder<Self>;
}
impl OpenTschunk for BB {
    type Open = OpenBB;
}
impl Tschunk for CC {
    type TVecBuilder = BufferTVecBuilder<Self>;
}
impl OpenTschunk for CC {
    type Open = OpenCC;
}
impl Tschunk for DD {
    type TVecBuilder = LeafTVecBuilder<Self>;
}

struct Root<T> {
    buf: Vec<u8>,
    offs: usize,
    _phantom: ::std::marker::PhantomData<T>,
}

struct OpenAA {
    b: <BB as Tschunk>::TVecBuilder,
    d: <DD as Tschunk>::TVecBuilder,
}

impl Default for OpenAA {
    fn default() -> Self {
        OpenAA {
            b: Default::default(),
            d: Default::default(),
        }
    }
}

struct OpenBB {
    c: <CC as Tschunk>::TVecBuilder,
}

impl Default for OpenBB {
    fn default() -> Self {
        OpenBB {
            c: Default::default(),
        }
    }
}

struct OpenCC {
    d: <DD as Tschunk>::TVecBuilder,
}

impl Default for OpenCC {
    fn default() -> Self {
        OpenCC {
            d: Default::default(),
        }
    }
}

impl AA {
    pub fn start_root(mut buf: Vec<u8>) -> Root<OpenAA> {
        use std::io::Write;
        unsafe {
            buf.write(&std::mem::transmute::<usize, [u8; 8]>(0usize)).unwrap();
        }
        Root {
            buf,
            offs: 8,
            _phantom: ::std::marker::PhantomData,
        }
    }

}


fn main() {
    let v = Vec::new();
    let mut aa = AA::start_root(v);
    let mut open_aa: OpenAA = Default::default();
    let mut open_b = open_aa.b.start_vec();
    let mut open_d = open_b.start_item().c.start_vec().start_item().d;
    open_d.start_vec(&mut aa).done();
    // open_b.done();
    open_aa.d.start_vec(&mut aa).done();

    // let a: OpenAA = AA::start();
    // let b: &mut OpenBB = a.start_b();
    // let c: &mut OpenCC = b.start_c();
    // c.push(CC { e: 12 });
    // c.done();
}
