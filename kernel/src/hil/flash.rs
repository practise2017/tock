//! Interface for reading, writing, and erasing flash storage pages.
//!
//! Operates on single pages. The page size is set by the associated type
//! `page`. Here is an example of a page type:
//!
//! ```rust
//! // Size in bytes
//! const PAGE_SIZE: u32 = 1024;
//!
//! pub struct NewChipPage(pub [u8; PAGE_SIZE as usize]);
//!
//! impl NewChipPage {
//!     pub const fn new() -> NewChipPage {
//!         NewChipPage([0; PAGE_SIZE as usize])
//!     }
//!
//!     fn len(&self) -> usize {
//!         self.0.len()
//!     }
//! }
//!
//! impl Index<usize> for NewChipPage {
//!     type Output = u8;
//!
//!     fn index(&self, idx: usize) -> &u8 {
//!         &self.0[idx]
//!     }
//! }
//!
//! impl IndexMut<usize> for NewChipPage {
//!     fn index_mut(&mut self, idx: usize) -> &mut u8 {
//!         &mut self.0[idx]
//!     }
//! }
//!
//! impl AsMut<[u8]> for NewChipPage {
//!     fn as_mut(&mut self) -> &mut [u8] {
//!         &mut self.0
//!     }
//! }
//! ```
//!
//! Then a basic implementation of this trait should look like:
//!
//! ```rust
//! impl hil::flash::Flash for NewChipStruct {
//!     type Page = NewChipPage;
//!
//!     fn set_client(&self, client: &'static hil::flash::Client<Self>) { }
//!     fn read_page(&self, page_number: usize, buf: &'static mut Self::Page) -> ReturnCode { }
//!     fn write_page(&self, page_number: usize, buf: &'static mut Self::Page) -> ReturnCode { }
//!     fn erase_page(&self, page_number: usize) -> ReturnCode { }
//! }
//! ```
//!
//! A user of this flash interface might look like:
//!
//! ```rust
//! pub struct FlashUser<'a, F: hil::flash::Flash + 'static> {
//!     driver: &'a F,
//!     buffer: TakeCell<'static, F::Page>,
//! }
//!
//! impl<'a, F: hil::flash::Flash + 'a> FlashUser<'a, F> {
//!     pub fn new(driver: &'a F, buffer: &'static mut F::Page) -> FlashUser<'a, F> {
//!         FlashUser {
//!             driver: driver,
//!             buffer: TakeCell::new(buffer),
//!         }
//!     }
//! }
//!
//! impl<'a, F: hil::flash::Flash + 'a> hil::flash::Client<F> for FlashUser<'a, F> {
//!     fn read_complete(&self, buffer: &'static mut F::Page, error: hil::flash::Error) {}
//!     fn write_complete(&self, buffer: &'static mut F::Page, error: hil::flash::Error) { }
//!     fn erase_complete(&self, error: hil::flash::Error) {}
//! }
//! ```

use returncode::ReturnCode;

/// Flash errors returned in the callbacks.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// Success.
    CommandComplete,

    /// An error occurred during the flash operation.
    FlashError,
}

/// A page of writable persistent flash memory.
pub trait Flash {
    /// Type of a single flash page for the given implementation.
    type Page: AsMut<[u8]>;

    /// Set the client for this flash peripheral. The client will be called
    /// when operations complete.
    fn set_client(&self, client: &'static Client<Self>);

    /// Read a page of flash into the buffer.
    fn read_page(&self, page_number: usize, buf: &'static mut Self::Page) -> ReturnCode;

    /// Write a page of flash from the buffer.
    fn write_page(&self, page_number: usize, buf: &'static mut Self::Page) -> ReturnCode;

    /// Erase a page of flash.
    fn erase_page(&self, page_number: usize) -> ReturnCode;
}

/// Implement `Client` to receive callbacks from `Flash`.
pub trait Client<F: Flash> {
    /// Flash read complete.
    fn read_complete(&self, read_buffer: &'static mut F::Page, error: Error);

    /// Flash write complete.
    fn write_complete(&self, write_buffer: &'static mut F::Page, error: Error);

    /// Flash erase complete.
    fn erase_complete(&self, error: Error);
}
