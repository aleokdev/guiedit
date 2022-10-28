use sfml::{
    graphics::{
        CircleShape, Color, ConvexShape, CustomShape, Drawable, IntRect, PrimitiveType,
        RectangleShape, RenderStates, RenderTarget, Sprite, Text, Vertex, VertexBuffer, View,
    },
    system::{SfStrConv, Vector2f, Vector2i, Vector2u},
    window::{ContextSettings, Cursor, Event, Handle, Style, VideoMode},
};

use sfml::graphics::RenderWindow as SfRenderWindow;

/// Wrapper over SFML's `RenderWindow`, with editor hooks set up.
#[derive(Debug)]
pub struct RenderWindow {
    render_window: SfRenderWindow,
}

impl RenderWindow {
    /// Construct a new render window
    ///
    /// This function creates the render window with the size and pixel
    /// depth defined in mode. An optional style can be passed to
    /// customize the look and behaviour of the window (borders,
    /// title bar, resizable, closable, ...). If style contains
    /// [`Style::FULLSCREEN`], then mode must be a valid video mode.
    ///
    /// The fourth parameter is a pointer to a structure specifying
    /// advanced OpenGL context settings such as antialiasing,
    /// depth-buffer bits, etc.
    ///
    /// # Arguments
    /// * mode - Video mode to use (defines the width, height and depth of the
    ///                             rendering area of the render window)
    /// * title - Title of the render window
    /// * style - Window style
    /// * settings - Additional settings for the underlying OpenGL context
    ///
    /// # Usage example
    ///
    /// ```no_run
    /// use sfml::window::Style;
    /// use sfml::graphics::{RenderWindow};
    /// // Create a new window
    /// let mut window = RenderWindow::new((800, 600),
    ///                              "SFML window",
    ///                              Style::CLOSE,
    ///                              &Default::default());
    /// ```
    pub fn new<V: Into<VideoMode>, S: SfStrConv>(
        mode: V,
        title: S,
        style: Style,
        settings: &ContextSettings,
    ) -> RenderWindow {
        let mut x = Self {
            render_window: SfRenderWindow::new(mode, title, style, settings),
        };

        // HACK: TEST
        // Maybe better to render the editor on another window? Still need to draw overlay and control events on original one though so maybe not
        let view = x.render_window.view();
        let mut view = sfml::graphics::View::new(view.center(), view.size());
        view.zoom(1.5);
        x.set_view(&view);
        x
    }

    /// Create a render window from an existing platform-specific window handle
    ///
    /// This function creates a render window based on an existing platform
    /// specific window handle which has been allocated outside of SFML. This is
    /// only intended to be used in cases where you need to integrate SFML with
    /// some other windowing library.
    ///
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that it is called with a valid window handle.
    ///
    /// # Arguments
    /// * handle - The handle to the platform-specific window handle to use for
    ///            the window.
    /// * settings - Additional settings for the underlying OpenGL context
    #[must_use]
    pub unsafe fn from_handle(handle: Handle, settings: &ContextSettings) -> RenderWindow {
        Self {
            render_window: SfRenderWindow::from_handle(handle, settings),
        }
    }

    /// Get the OS-specific handle of the window.
    ///
    /// The type of the returned handle is Handle, which is a typedef to the handle type defined by the OS.
    /// You shouldn't need to use this function, unless you have very specific stuff to implement that SFML
    /// doesn't support, or implement a temporary workaround until a bug is fixed.
    #[must_use]
    pub fn system_handle(&self) -> Handle {
        self.render_window.system_handle()
    }

    /// Change a render window's icon
    /// pixels must be an array of width x height pixels in 32-bits RGBA format.
    ///
    /// # Arguments
    /// * width - Icon's width, in pixels
    /// * height - Icon's height, in pixels
    /// * pixels - Vector of pixels
    ///
    /// # Safety
    ///
    /// `pixels` not being at least `width * height * 4` will likely cause undefined behavior.
    ///
    /// Platform-specific behavior is also unclear (limits on max size, etc).
    ///
    /// # Usage example
    ///
    /// ```no_run
    /// # use sfml::window::Style;
    /// # use sfml::graphics::{RenderWindow};
    /// # // Create a new window
    /// # let mut window = RenderWindow::new((800, 600),
    /// #                              "SFML window",
    /// #                              Style::CLOSE,
    /// #                              &Default::default());
    /// while window.is_open() {
    /// // Creates a bright red window icon
    /// let (width, height) = (1, 1);
    /// let pixels: [u8; 4] = [255, 0, 0, 255];
    /// unsafe { window.set_icon(width, height, &pixels); }
    ///     window.display();
    /// }
    /// ```
    pub unsafe fn set_icon(&mut self, width: u32, height: u32, pixels: &[u8]) {
        self.render_window.set_icon(width, height, pixels)
    }

    /// Pop the event on top of event queue, if any, and return it
    ///
    /// This function is not blocking: if there's no pending event then
    /// it will return `None`.
    /// Note that more than one event may be present in the event queue,
    /// thus you should always call this function in a loop
    /// to make sure that you process every pending event.
    ///
    /// Returns `Some(event)` if an event was returned, or `None` if the event queue was empty
    ///
    /// # Usage example
    ///
    /// ```no_run
    /// # use sfml::window::{Event, Style};
    /// # use sfml::graphics::RenderWindow;
    /// # // Create a new window
    /// # let mut window = RenderWindow::new((800, 600),
    /// #                              "SFML window",
    /// #                              Style::CLOSE,
    /// #                              &Default::default());
    /// while window.is_open() {
    ///     // Event processing
    ///     while let Some(event) = window.poll_event() {
    ///         match event {
    ///             Event::Closed => window.close(),
    ///             _ => {},
    ///         }
    ///     }
    /// }
    /// ```
    pub fn poll_event(&mut self) -> Option<Event> {
        self.render_window.poll_event()
    }

    /// Wait for an event and return it
    ///
    /// This function is blocking: if there's no pending event then
    /// it will wait until an event is received.
    ///
    /// This function is typically used when you have a thread that
    /// is dedicated to events handling: you want to make this thread
    /// sleep as long as no new event is received.
    ///
    /// Returns `Some(event)` or `None` if an error has occured
    ///
    /// # Usage example
    ///
    /// ```no_run
    /// # use sfml::window::{Event, Style};
    /// # use sfml::graphics::RenderWindow;
    /// # // Create a new window
    /// # let mut window = RenderWindow::new((800, 600),
    /// #                              "SFML window",
    /// #                              Style::CLOSE,
    /// #                              &Default::default());
    /// // The main loop - ends as soon as the window is closed
    /// while window.is_open() {
    ///     // Event processing
    ///     match window.wait_event() { // Stops program from continuing until new event occurs
    ///         Some(Event::Closed) => window.close(),
    ///         _ => {},
    ///     }
    /// }
    /// ```
    pub fn wait_event(&mut self) -> Option<Event> {
        self.render_window.wait_event()
    }

    /// Close a render window and destroy all the attached resources
    ///
    /// After calling this method, the Window object remains
    /// valid.
    /// All other functions such as `poll_event` or display
    /// will still work (i.e. you don't have to test `is_open`
    /// every time), and will have no effect on closed windows.
    ///
    /// # Usage Example
    ///
    /// ```no_run
    /// # use sfml::window::{Event, Style};
    /// # use sfml::graphics::RenderWindow;
    /// # // Create a new window
    /// # let mut window = RenderWindow::new((800, 600),
    /// #                              "SFML window",
    /// #                              Style::CLOSE,
    /// #                              &Default::default());
    /// // The main loop - ends as soon as the window is closed
    /// while window.is_open() {
    ///     // Event processing
    ///     while let Some(event) = window.poll_event() {
    ///         match event {
    ///             Event::Closed => window.close(),
    ///             _ => {}
    ///         }
    ///     }
    /// }
    /// // Once window is closed, we can do other things.
    /// ```
    pub fn close(&mut self) {
        self.render_window.close()
    }

    /// Tell whether or not a window is opened
    ///
    /// This function returns whether or not the window exists.
    /// Note that a hidden window `(set_visible(false))` will return
    /// true.
    ///
    /// # Usage Example
    ///
    /// ```no_run
    /// use sfml::window::{Event, Style};
    /// use sfml::graphics::RenderWindow;
    /// // Create a new window
    /// let mut window = RenderWindow::new((800, 600),
    ///                              "SFML window",
    ///                              Style::CLOSE,
    ///                              &Default::default());
    ///
    /// while window.is_open() {
    ///     // Do something
    /// }
    /// ```
    #[must_use]
    pub fn is_open(&self) -> bool {
        self.render_window.is_open()
    }

    /// Display on screen what has been rendered to the window so far
    ///
    /// This function is typically called after all OpenGL rendering
    /// has been done for the current frame, in order to show
    /// it on screen.
    ///
    /// # Usage Example
    ///
    /// ```no_run
    /// # use sfml::window::{Event, Style};
    /// # use sfml::graphics::{ RenderWindow, RenderTarget, Color };
    /// # // Create a new window
    /// # let mut window = RenderWindow::new((800, 600),
    /// #                              "SFML window",
    /// #                              Style::CLOSE,
    /// #                              &Default::default());
    /// while window.is_open() {
    ///     window.clear(Color::BLACK);
    ///     // Draw something
    ///
    ///     window.display();
    /// }
    /// ```
    pub fn display(&mut self) {
        self.render_window.display()
    }

    /// Limit the framerate to a maximum fixed frequency
    ///
    /// If a limit is set, the window will use a small delay after
    /// each call to [`RenderWindow::display`] to ensure that the current frame
    /// lasted long enough to match the framerate limit.
    ///
    /// # Arguments
    /// * limit - Framerate limit, in frames per seconds (use 0 to disable limit)
    pub fn set_framerate_limit(&mut self, limit: u32) {
        self.render_window.set_framerate_limit(limit)
    }

    /// Get the settings of the OpenGL context of a window
    ///
    /// Note that these settings may be different from what was
    /// passed to the [`RenderWindow::new`] function,
    /// if one or more settings were not supported. In this case,
    /// SFML chose the closest match.
    ///
    /// Return a structure containing the OpenGL context settings
    #[must_use]
    pub fn settings(&self) -> &ContextSettings {
        self.render_window.settings()
    }

    /// Change the title of a window
    ///
    /// # Arguments
    /// * title - New title
    pub fn set_title<S: SfStrConv>(&mut self, title: S) {
        self.render_window.set_title(title)
    }

    /// Show or hide a window.
    ///
    /// # Arguments
    /// * visible - true to show the window, false to hide it
    pub fn set_visible(&mut self, visible: bool) {
        self.render_window.set_visible(visible)
    }

    /// Show or hide the mouse cursor
    ///
    /// # Arguments
    /// * visible - true to  false to hide
    pub fn set_mouse_cursor_visible(&mut self, visible: bool) {
        self.render_window.set_mouse_cursor_visible(visible)
    }

    /// Grab or release the mouse cursor.
    ///
    /// If set, grabs the mouse cursor inside this window's client area so it may no longer be
    /// moved outside its bounds. Note that grabbing is only active while the window has focus.
    pub fn set_mouse_cursor_grabbed(&mut self, grabbed: bool) {
        self.render_window.set_mouse_cursor_grabbed(grabbed)
    }

    /// Enable or disable vertical synchronization
    ///
    /// Activating vertical synchronization will limit the number
    /// of frames displayed to the refresh rate of the monitor.
    /// This can avoid some visual artifacts, and limit the framerate
    /// to a good value (but not constant across different computers).
    ///
    /// # Arguments
    /// * enabled - true to enable v-sync, false to deactivate
    pub fn set_vertical_sync_enabled(&mut self, enabled: bool) {
        self.render_window.set_vertical_sync_enabled(enabled)
    }

    /// Enable or disable automatic key-repeat
    ///
    /// If key repeat is enabled, you will receive repeated
    /// [`crate::window::Event::KeyPressed`] events while keeping a key pressed.
    /// If it is disabled, you will only get a single event when the key is pressed.
    ///
    /// Key repeat is enabled by default.
    ///
    /// # Arguments
    /// * enabled - true to enable, false to disable
    pub fn set_key_repeat_enabled(&mut self, enabled: bool) {
        self.render_window.set_key_repeat_enabled(enabled)
    }

    /// Activate or deactivate a render window as the current target for OpenGL rendering
    ///
    /// A window is active only on the current thread, if you want to
    /// make it active on another thread you have to deactivate it
    /// on the previous thread first if it was active.
    /// Only one window can be active on a thread at a time, thus
    /// the window previously active (if any) automatically gets deactivated.
    ///
    /// # Arguments
    /// * active - true to activate, false to deactivate
    ///
    /// Return true if operation was successful, false otherwise
    pub fn set_active(&mut self, enabled: bool) -> bool {
        self.render_window.set_active(enabled)
    }

    /// Change the joystick threshold
    ///
    /// The joystick threshold is the value below which
    /// no [`crate::window::Event::JoystickMoved`] event will be generated.
    ///
    /// # Arguments
    /// * threshold - New threshold, in the range [0, 100]
    pub fn set_joystick_threshold(&mut self, threshold: f32) {
        self.render_window.set_joystick_threshold(threshold)
    }

    /// Get the position of a window
    ///
    /// Return the position in pixels
    #[must_use]
    pub fn position(&self) -> Vector2i {
        self.render_window.position()
    }

    /// Change the position of a window on screen
    ///
    /// This function only works for top-level windows
    /// (i.e. it will be ignored for windows created from
    /// the handle of a child window/control).
    ///
    /// # Arguments
    /// * position - New position of the window, in pixels
    ///
    /// # Usage Example
    ///
    /// ```no_run
    /// # use sfml::window::{Event, Style};
    /// # use sfml::graphics::RenderWindow;
    /// # use sfml::system::Vector2;
    /// # // Create a new window with SFML window as name
    /// # let mut window = RenderWindow::new((800, 600),
    /// #                              "SFML window",
    /// #                              Style::CLOSE,
    /// #                              &Default::default());
    /// window.set_position(Vector2::new(100, 400));
    /// use std::{thread, time::Duration};
    /// // You need to wait for the OS the set the window's position before checking
    /// thread::sleep(Duration::from_millis(250));
    /// assert_eq!(window.position(), Vector2::new(100, 400));
    /// ```
    pub fn set_position(&mut self, position: Vector2i) {
        self.render_window.set_position(position)
    }

    /// Change the size of the rendering region of a window
    ///
    /// # Arguments
    /// * size - New size, in pixels
    ///
    /// # Usage Example
    ///
    /// ```no_run
    /// # use sfml::window::{Event, Style};
    /// # use sfml::graphics::{ RenderWindow, RenderTarget };
    /// # use sfml::system::Vector2;
    /// # // Create a new window with SFML window as name
    /// # let mut window = RenderWindow::new((800, 600),
    /// #                              "SFML window",
    /// #                              Style::CLOSE,
    /// #                              &Default::default());
    /// window.set_size(Vector2::new(100, 400));
    /// use std::{thread, time::Duration};
    /// // You need to wait for the OS the set the window's size before checking
    /// thread::sleep(Duration::from_millis(250));
    /// assert_eq!(window.size(), Vector2::new(100, 400));
    /// ```
    pub fn set_size<S: Into<Vector2u>>(&mut self, size: S) {
        self.render_window.set_size(size)
    }

    /// Returns the current position of the mouse relative to the window.
    #[must_use]
    pub fn mouse_position(&self) -> Vector2i {
        self.render_window.mouse_position()
    }

    /// Set the current position of the mouse relatively to a render window
    ///
    /// This function sets the current position of the mouse cursor relative
    /// to the given render window
    ///
    /// # Arguments
    /// * `position` - the positon to set
    pub fn set_mouse_position(&mut self, position: Vector2i) {
        self.render_window.set_mouse_position(position)
    }

    /// Set the displayed cursor to a native system cursor.
    ///
    /// Upon window creation, the arrow cursor is used by default.
    ///
    /// # Safety
    ///
    /// The cursor can not be destroyed while in use by the window.
    ///
    /// # Usage Example
    ///
    /// ```no_run
    /// # use sfml::window::{Event, Style};
    /// # use sfml::graphics::RenderWindow;
    /// # // Create a new window with SFML window as name
    /// # let mut window = RenderWindow::new((800, 600),
    /// #                              "SFML window",
    /// #                              Style::CLOSE,
    /// #                              &Default::default());
    /// # use sfml::window::{ Cursor, CursorType };
    /// let cursor = Cursor::from_system(CursorType::Arrow);
    /// if let Some(arrow_cursor) = &cursor {
    ///     unsafe { window.set_mouse_cursor(arrow_cursor); }
    /// }
    /// // You need to ensure the SFML window closes before the cursor's end of life.
    /// // Doing it the other way around will cause undefined behavior.
    /// window.close();
    /// drop(cursor);
    /// ```
    pub unsafe fn set_mouse_cursor(&mut self, cursor: &Cursor) {
        self.render_window.set_mouse_cursor(cursor)
    }

    /// Returns the current position of a touch in window coordinates.
    #[must_use]
    pub fn touch_position(&self, finger: u32) -> Vector2i {
        self.render_window.touch_position(finger)
    }

    /// Check whether the window has the input focus.
    ///
    /// At any given time, only one window may have the input focus to receive input events
    /// such as keystrokes or most mouse events.
    #[must_use]
    pub fn has_focus(&self) -> bool {
        self.render_window.has_focus()
    }

    /// Request the current window to be made the active foreground window.
    ///
    /// At any given time, only one window may have the input focus to receive input events
    /// such as keystrokes or mouse events. If a window requests focus, it only hints to the
    /// operating system, that it would like to be focused. The operating system is free to
    /// deny the request. This is not to be confused with [`RenderWindow::set_active`].
    ///
    /// # Usage Example
    ///
    /// ```no_run
    /// # use sfml::window::{Event, Style};
    /// # use sfml::graphics::RenderWindow;
    /// # // Create a new window with SFML window as name
    /// # let mut window = RenderWindow::new((800, 600),
    /// #                              "SFML window",
    /// #                              Style::CLOSE,
    /// #                              &Default::default());
    /// window.request_focus();
    /// use std::{thread, time::Duration};
    /// // You need to wait for the OS the set the window's visibility before checking
    /// thread::sleep(Duration::from_millis(250));
    /// assert_eq!(window.has_focus(), true);
    /// ```
    pub fn request_focus(&self) {
        self.render_window.request_focus()
    }
}

impl RenderTarget for RenderWindow {
    fn push_gl_states(&mut self) {
        self.render_window.push_gl_states()
    }
    fn pop_gl_states(&mut self) {
        self.render_window.pop_gl_states()
    }
    fn reset_gl_states(&mut self) {
        self.render_window.reset_gl_states()
    }
    fn set_view(&mut self, view: &View) {
        self.render_window.set_view(view)
    }
    fn view(&self) -> &View {
        self.render_window.view()
    }
    fn default_view(&self) -> &View {
        self.render_window.default_view()
    }
    fn map_pixel_to_coords(&self, point: Vector2i, view: &View) -> Vector2f {
        self.render_window.map_pixel_to_coords(point, view)
    }
    fn map_pixel_to_coords_current_view(&self, point: Vector2i) -> Vector2f {
        self.render_window.map_pixel_to_coords_current_view(point)
    }
    fn map_coords_to_pixel(&self, point: Vector2f, view: &View) -> Vector2i {
        self.render_window.map_coords_to_pixel(point, view)
    }
    fn map_coords_to_pixel_current_view(&self, point: Vector2f) -> Vector2i {
        self.render_window.map_coords_to_pixel_current_view(point)
    }
    fn viewport(&self, view: &View) -> IntRect {
        self.render_window.viewport(view)
    }
    fn size(&self) -> Vector2u {
        self.render_window.size()
    }
    fn draw(&mut self, object: &dyn Drawable) {
        self.render_window.draw(object)
    }
    fn draw_with_renderstates(&mut self, object: &dyn Drawable, render_states: &RenderStates) {
        self.render_window
            .draw_with_renderstates(object, render_states)
    }
    fn draw_text(&self, text: &Text, render_states: &RenderStates) {
        self.render_window.draw_text(text, render_states)
    }
    fn draw_shape(&self, shape: &CustomShape, render_states: &RenderStates) {
        self.render_window.draw_shape(shape, render_states)
    }
    fn draw_sprite(&self, sprite: &Sprite, render_states: &RenderStates) {
        self.render_window.draw_sprite(sprite, render_states)
    }
    fn draw_circle_shape(&self, circle_shape: &CircleShape, render_states: &RenderStates) {
        self.render_window
            .draw_circle_shape(circle_shape, render_states)
    }
    fn draw_rectangle_shape(&self, rectangle_shape: &RectangleShape, render_states: &RenderStates) {
        self.render_window
            .draw_rectangle_shape(rectangle_shape, render_states)
    }
    fn draw_convex_shape(&self, convex_shape: &ConvexShape, render_states: &RenderStates) {
        self.render_window
            .draw_convex_shape(convex_shape, render_states)
    }
    fn draw_vertex_buffer(&self, vertex_buffer: &VertexBuffer, render_states: &RenderStates) {
        self.render_window
            .draw_vertex_buffer(vertex_buffer, render_states)
    }
    fn draw_primitives(&self, vertices: &[Vertex], ty: PrimitiveType, rs: &RenderStates) {
        self.render_window.draw_primitives(vertices, ty, rs)
    }
    fn clear(&mut self, color: Color) {
        self.render_window.clear(color)
    }
}
