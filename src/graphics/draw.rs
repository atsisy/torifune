use std::cell::RefCell;

use ggez::graphics as ggraphics;

use super::numeric;

#[derive(Debug, Clone, Copy)]
pub struct StackScreen {
    position: numeric::Point2f,
    size: numeric::Vector2u,
}

impl StackScreen {
    pub fn end_point(&self) -> numeric::Point2f {
	numeric::Point2f::new(
	    self.position.x + self.size.x as f32,
	    self.position.y + self.size.y as f32,
	)
    }
}

thread_local!(static SCREEN_STACK: RefCell<Vec<StackScreen>> = {
    RefCell::new(Vec::new())
});

thread_local!(static TARGET_SCREEN: RefCell<StackScreen> = {
    RefCell::new(StackScreen {
	position: numeric::Point2f::new(0.0, 0.0),
	size: numeric::Vector2u::new(0, 0),
    })
});

pub fn reset_stacking_screen(screen: StackScreen) {
    TARGET_SCREEN.with(|target_screen| {
	target_screen.swap(&RefCell::new(screen));
    });
}

pub fn draw<D>(ctx: &mut ggez::Context, drawable: &D, mut params: ggraphics::DrawParam) -> ggez::GameResult<()>
where D: ggraphics::Drawable {
    TARGET_SCREEN.with(|target_screen| {
	let original_dest = params.dest;
	let mut begin_src = numeric::Vector2f::new(0.0, 0.0);
	let mut end_src = numeric::Vector2f::new(1.0, 1.0);
	
	params.dest.x += target_screen.borrow_mut().position.x;
	params.dest.y += target_screen.borrow_mut().position.y;

	let bounds = drawable.dimensions(ctx);

	if let Some(bounds) = bounds {
	    if original_dest.x < 0.0 {
		begin_src.x -= original_dest.x / bounds.w;
	    }
	    
	    if original_dest.y < 0.0 {
		begin_src.y -= original_dest.y / bounds.h;
	    }
	    
	    let screen_end_point = target_screen.borrow().end_point();
	    
	    if original_dest.x > screen_end_point.x {
		end_src.x -= (original_dest.x - screen_end_point.x) / bounds.w;
	    }
	    
	    if original_dest.y > screen_end_point.y {
		end_src.y -= (original_dest.y - screen_end_point.y) / bounds.h;
	    }
	    params.src = numeric::Rect::new(begin_src.x, begin_src.y, end_src.x, end_src.y);
	}
    });
    ggraphics::draw(ctx, drawable, params)
}

pub fn stack_screen(new_screen: StackScreen) {
    TARGET_SCREEN.with(|target_screen| {
	SCREEN_STACK.with(|stack| {
	    stack.borrow_mut().push(target_screen.borrow().clone());
	});
	
	let borrowed_screen: &mut StackScreen = &mut target_screen.borrow_mut();
	let current_end_point = borrowed_screen.end_point();
	let new_end_point = borrowed_screen.end_point();
	
	borrowed_screen.position.x += new_screen.position.x;
	borrowed_screen.position.y += new_screen.position.y;

	if current_end_point.x >= new_end_point.x {
	    borrowed_screen.size.x -= (current_end_point.x - new_end_point.x) as u32;
	}

	if current_end_point.y >= new_end_point.y {
	    borrowed_screen.size.y -= (current_end_point.y - new_end_point.y) as u32;
	}
    });

    SCREEN_STACK.with(|stack| {
	stack.borrow_mut().push(new_screen);
    });
}

pub fn pop_screen() -> Option<StackScreen> {
    let last_arg_screen = SCREEN_STACK.with(|stack| stack.borrow_mut().pop());
    let last_cur_screen = SCREEN_STACK.with(|stack| stack.borrow_mut().pop());

    if last_cur_screen.is_some() && last_arg_screen.is_some() {
	TARGET_SCREEN.with(|target_screen| {
	    if let Some(last_current_screen) = last_cur_screen {
		target_screen.swap(&RefCell::new(last_current_screen));
	    } else {
		target_screen.swap(&RefCell::new(StackScreen {
		    position: numeric::Point2f::new(0.0, 0.0),
		    size: numeric::Vector2u::new(0, 0),
		}));
	    };
	});

	last_arg_screen
    } else {
	None
    }
}
