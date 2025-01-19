use slotmap::HopSlotMap;

use crate::{graphics::Rect, tree::Tree, TabId};

#[derive(Debug)]
pub struct Tab {
    pub name: String,
    pub tree: Tree,
    pub focused_at: std::time::Instant,
}

impl Tab {
    /// Mark tab as recent; used for MRU sorting
    pub fn mark_as_focused(&mut self) {
        self.focused_at = std::time::Instant::now();
    }
}

#[derive(Debug)]
pub struct Tabs {
    focus: TabId,
    tabs: HopSlotMap<TabId, Tab>,
}

impl Tabs {
    #[inline]
    pub fn new(area: Rect) -> Self {
        let mut tabs = HopSlotMap::with_key();
        let tab = Tab {
            name: "Tab 0".to_string(),
            tree: Tree::new(area),
            focused_at: std::time::Instant::now(),
        };
        let focus = tabs.insert(tab);
        Self { focus, tabs }
    }

    #[inline]
    pub fn curr_tree_mut(&mut self) -> &mut Tree {
        &mut self.tabs.get_mut(self.focus).unwrap().tree
    }

    #[inline]
    pub fn curr_tree(&self) -> &Tree {
        &self.tabs.get(self.focus).unwrap().tree
    }

    #[inline]
    pub fn iter_tabs_mut(&mut self) -> impl Iterator<Item = (TabId, &mut Tab)> {
        self.tabs.iter_mut()
    }

    #[inline]
    pub fn iter_tabs(&self) -> impl Iterator<Item = (TabId, &Tab)> {
        self.tabs.iter()
    }

    #[inline]
    pub fn curr_tab_mut(&mut self) -> &mut Tab {
        self.tabs.get_mut(self.focus).unwrap()
    }

    #[inline]
    pub fn set_focus(&mut self, id: TabId) {
        self.focus = id;
        self.tabs[id].mark_as_focused();
    }

    #[inline]
    pub fn focus(&self) -> TabId {
        self.focus
    }

    #[inline]
    pub fn new_tab(&mut self) -> TabId {
        let area = self.curr_tree().area();
        let new_tab = Tab {
            name: format!("Tab {}", self.tabs.len()),
            tree: Tree::new(area),
            focused_at: std::time::Instant::now(),
        };
        let new_focus = self.tabs.insert(new_tab);
        self.set_focus(new_focus);
        new_focus
    }

    #[inline]
    pub fn focus_next(&mut self) -> TabId {
        let curr = self.focus;
        let mut iter = self.tabs.keys().skip_while(|id| *id != curr);
        iter.next();
        let id = iter.next().or_else(|| self.tabs.keys().next()).unwrap();
        self.set_focus(id);
        id
    }

    #[inline]
    pub fn focus_previous(&mut self) -> TabId {
        let curr = self.focus;
        let iter = self.tabs.keys().take_while(|id| *id != curr);
        let id = iter.last().or_else(|| self.tabs.keys().last()).unwrap();
        self.set_focus(id);
        id
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.tabs.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.tabs.is_empty()
    }
}
