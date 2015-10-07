extern crate kafka;

#[test]
fn it_works() {

}

#[test]
#[cfg(feature="mio-support")]
fn it_works_mio() {

}

#[test]
#[cfg(all(feature="mio-support", feature="integration"))]
fn it_works_mio_integration() {

}