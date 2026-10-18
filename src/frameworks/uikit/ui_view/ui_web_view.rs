/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UIWebView`.

use crate::frameworks::core_graphics::CGRect;
use crate::frameworks::foundation::ns_string::to_rust_string;
use crate::msg;
use crate::objc::{id, msg_class, msg_super, nil, objc_classes, release, ClassExports};
use std::borrow::Cow;

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIWebView: UIView

- (id)initWithFrame:(CGRect)frame {
    let this: id = msg_super![env; this initWithFrame:frame];
    // The page is shown in a scroll view, the web view's first subview.
    // Some games reach it through -subviews to hide its shadows.
    let bounds: CGRect = msg![env; this bounds];
    let scroll_view: id = msg_class![env; UIScrollView alloc];
    let scroll_view: id = msg![env; scroll_view initWithFrame:bounds];
    () = msg![env; this addSubview:scroll_view];
    release(env, scroll_view);
    this
}

// NSCoding implementation
- (id)initWithCoder:(id)_coder {
    todo!()
}

- (())setScalesPageToFit:(bool)_scales {
    // TODO
}
- (())setDelegate:(id)_delegate {
    // TODO
}
- (())loadRequest:(id)request { // NSURLRequest*
    let url_string = if request != nil {
        let url = msg![env; request URL];
        let url_desc = msg![env; url description];
        to_rust_string(env, url_desc)
    } else {
        Cow::default()
    };
    log!("TODO: [(UIWebView*) {:?} loadRequest:{:?} ({})]", this, request, url_string);
}

@end

};
