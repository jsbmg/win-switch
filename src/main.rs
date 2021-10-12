use std::iter::{Iterator};

extern crate xcb;

use xcb::xproto;

fn main() {
    if let Ok((conn, screen_num)) = xcb::Connection::connect(None) {
	println!("Connected to X on screen {}!", screen_num);
	let root_window = get_root_window(&conn, screen_num);
	let net_client_list = xcb::xproto::intern_atom(&conn, true, "_NET_CLIENT_LIST")
	    .get_reply()
	    .unwrap();
	let window_list = xcb::get_property(&conn, false, root_window, net_client_list.atom(), xproto::GET_PROPERTY_TYPE_ANY, 0, u32::MAX);
	if let Ok(windows) = window_list.get_reply() {
	    let windows = windows.value::<u32>();
	    println!("{:?}", &windows);
	    let new = windows[0];
	    let values = [(xproto::CONFIG_WINDOW_STACK_MODE as u16, xproto::STACK_MODE_ABOVE)];
	    xproto::configure_window(&conn,
				     new,
				     &values);
	    for w in windows {
		print_window_name(&conn, *w)
	    }
	}
    } else {
	println!("Could not connect to X!");
    }
}
fn get_root_window(conn: &xcb::Connection, screen_num: i32) -> u32 {
    // returns the id of the root window
    let setup = &conn.get_setup();
    let root_window = setup
	.roots()
	.nth(screen_num as usize)
	.unwrap()
	.root();
    root_window
}

fn print_window_name(conn: &xcb::Connection, w: u32) {
    let cookie = xcb::get_property(&conn,
				   false,
				   w as u32,
				   xcb::ATOM_WM_NAME,
				   xcb::ATOM_STRING,
				   0,
				   1024);
    if let Ok(reply) = cookie.get_reply() {
	let mystring = std::str::from_utf8(reply.value()).unwrap();
	println!("{:?}", mystring);
    } else {
	println!("Could not get information about the window.")
    }
}

    
