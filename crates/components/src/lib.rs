//! gpui-component-style component library for [`egui`] 0.34, ported from
//! [`gpui-component`](https://github.com/longbridge/gpui-component).
//!
//! All components are idiomatic egui widgets that return [`egui::Response`].
//! Most can be added with `ui.add(Button::primary("OK"))`; a few that need
//! to return extra info (e.g. [`Tag::show`]) provide a `.show(ui)` method.
//!
//! Theming comes from [`egui_components_theme::Theme`]. Install it once at
//! startup:
//!
//! ```no_run
//! # let ctx = egui::Context::default();
//! egui_components_theme::Theme::dark().install(&ctx);
//! ```
//!
//! Components read the installed theme via [`egui_components_theme::Theme::get`].

pub mod accordion;
pub mod alert;
pub mod avatar;
pub mod badge;
pub mod breadcrumb;
pub mod button;
pub mod card;
pub mod checkbox;
pub mod collapsible;
pub mod common;
pub mod description_list;
pub mod dialog;
pub mod form;
pub mod heading;
pub mod hover_card;
pub mod icon;
pub mod input;
pub mod label;
pub mod link;
pub mod list;
pub mod menu;
pub mod notification;
pub mod number_input;
pub mod otp_input;
pub mod pagination;
pub mod popover;
pub mod progress;
pub mod radio;
pub mod rating;
pub mod resizable;
pub mod scroll_area;
pub mod select;
pub mod separator;
pub mod sidebar;
pub mod slider;
pub mod switch;
pub mod tabs;
pub mod tag;
pub mod titlebar;
pub mod tooltip;
pub mod tree;

pub use accordion::Accordion;
pub use alert::Alert;
pub use avatar::{Avatar, AvatarShape, AvatarStatus};
pub use badge::Badge;
pub use breadcrumb::Breadcrumb;
pub use button::Button;
pub use card::{Card, CardVariant};
pub use checkbox::Checkbox;
pub use collapsible::Collapsible;
pub use common::{Size, Variant};
pub use description_list::DescriptionList;
pub use dialog::{AlertChoice, AlertDialog, Dialog};
pub use form::{Form, FormUi};
pub use heading::{Heading, HeadingLevel};
pub use hover_card::HoverCard;
pub use icon::{Icon, IconKind};
pub use input::Input;
pub use label::{Label, LabelTone};
pub use link::Link;
pub use list::{List, ListItem};
pub use menu::Menu;
pub use notification::{ToastAnchor, Toasts};
pub use number_input::NumberInput;
pub use otp_input::OtpInput;
pub use pagination::Pagination;
pub use popover::Popover;
pub use progress::Progress;
pub use radio::Radio;
pub use rating::Rating;
pub use resizable::Resizable;
pub use scroll_area::ScrollArea;
pub use select::Select;
pub use separator::Separator;
pub use sidebar::{Rail, RailUi, Sidebar, SidebarUi};
pub use slider::Slider;
pub use switch::Switch;
pub use tabs::{TabVariant, Tabs};
pub use tag::{Tag, TagResponse};
pub use titlebar::TitleBar;
pub use tooltip::Tooltip;
pub use tree::{
    show_themed as show_themed_tree, Tree, TreeAction, TreeView, TreeViewBuilder,
    TreeViewSettings, TreeViewState,
};

pub use egui_components_theme as theme;
