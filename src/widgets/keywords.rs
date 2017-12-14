use gtk;
use gtk::prelude::*;
use relm_attributes::widget;

#[derive(Msg)]
pub enum Msg {
    Add,
    Delete,
    Edit(Column, ::gtk::TreePath, String),
    Set(::std::collections::BTreeMap<String, String>),
    Updated(::std::collections::BTreeMap<String, String>),
}

pub struct Model {
    store: ::gtk::ListStore,
    relm: ::relm::Relm<Keywords>,
}

#[repr(u32)]
#[derive(Clone)]
pub enum Column {
    Name = 0,
    Value = 1,
}

impl ::std::convert::Into<u32> for Column
{
    fn into(self) -> u32
    {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

impl ::std::convert::Into<i32> for Column
{
    fn into(self) -> i32
    {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

impl Keywords
{
    fn add(&mut self)
    {
        let iter = self.model.store.append();
        let path = self.model.store.get_path(&iter)
            .unwrap();
        let column = self.tree_view.get_column(Column::Name.into());

        self.tree_view.set_cursor(&path, column.as_ref(), true);
    }

    fn delete(&mut self)
    {
        if let Some((_, iter)) = self.tree_view.get_selection().get_selected() {
            self.model.store.remove(&iter);

            self.model.relm.stream()
                .emit(Msg::Updated(self.keywords()));
        }
    }

    fn edit(&mut self, column: Column, path: &::gtk::TreePath, new_text: &str)
    {
        let iter = self.model.store.get_iter(path)
            .unwrap();

        self.model.store.set_value(&iter, column.into(), &new_text.to_value());

        self.model.relm.stream()
            .emit(Msg::Updated(self.keywords()));
    }

    fn keywords(&self) -> ::std::collections::BTreeMap<String, String>
    {
        let mut keywords = ::std::collections::BTreeMap::new();

        let iter = match self.model.store.get_iter_first() {
            Some(iter) => iter,
            None => return keywords,
        };

        loop {
            let name = match self.model.store.get_value(&iter, Column::Name.into()).get() {
                Some(name) => name,
                None => break,
            };
            let value = match self.model.store.get_value(&iter, Column::Value.into()).get() {
                Some(value) => value,
                None => break,
            };

            keywords.insert(name, value);

            if !self.model.store.iter_next(&iter) {
                break;
            }
        }

        keywords
    }

    fn set(&mut self, keywords: &::std::collections::BTreeMap<String, String>)
    {
        self.model.store.clear();

        for (name, value) in keywords {
            let row = self.model.store.append();
            self.model.store.set_value(&row, Column::Name.into(), &name.to_value());
            self.model.store.set_value(&row, Column::Value.into(), &value.to_value());
        }
    }
}

#[widget]
impl ::relm::Widget for Keywords
{
    fn init_view(&mut self)
    {
        self.scroll.set_policy(::gtk::PolicyType::Never, ::gtk::PolicyType::Automatic);
        self.scroll.set_property_height_request(250);
        self.tree_view.set_model(Some(&self.model.store));

        let column = ::gtk::TreeViewColumn::new();
        column.set_title("name");
        self.tree_view.append_column(&column);

        let cell = ::gtk::CellRendererText::new();
        cell.set_property_editable(true);
        connect!(self.model.relm, cell, connect_edited(_, path, new_text), Msg::Edit(Column::Name, path, new_text.to_owned()));
        column.pack_start(&cell, true);
        column.add_attribute(&cell, "text", Column::Name.into());

        let column = ::gtk::TreeViewColumn::new();
        column.set_title("value");
        self.tree_view.append_column(&column);

        let cell = ::gtk::CellRendererText::new();
        cell.set_property_editable(true);
        connect!(self.model.relm, cell, connect_edited(_, path, new_text), Msg::Edit(Column::Value, path, new_text.to_owned()));
        column.pack_start(&cell, true);
        column.add_attribute(&cell, "text", Column::Value.into());
    }

    fn model(relm: &::relm::Relm<Self>, _: ()) -> Model
    {
        let columns = vec![
            ::gtk::Type::String,
            ::gtk::Type::String,
        ];

        Model {
            store: ::gtk::ListStore::new(&columns),
            relm: relm.clone(),
        }
    }

    fn update(&mut self, event: Msg)
    {
        use self::Msg::*;

        match event {
            Add => self.add(),
            Delete => self.delete(),
            Edit(ref column, ref path, ref new_text) => self.edit(column.clone(), path, new_text),
            Set(keywords) => self.set(&keywords),
            Updated(_) => (),
        }
    }

    view!
    {
        gtk::Box {
            orientation: ::gtk::Orientation::Vertical,
            #[name="scroll"]
            gtk::ScrolledWindow {
                packing: {
                    expand: true,
                    fill: true,
                },
                #[name="tree_view"]
                gtk::TreeView {
                    headers_visible: true,
                },
            },
            gtk::ActionBar {
                packing: {
                    expand: false,
                    fill: true,
                },
                gtk::Button {
                    image: &::gtk::Image::new_from_icon_name("list-add", ::gtk::IconSize::SmallToolbar.into()),
                    clicked => Msg::Add,
                },
                gtk::Button {
                    image: &::gtk::Image::new_from_icon_name("list-remove", ::gtk::IconSize::SmallToolbar.into()),
                    clicked => Msg::Delete,
                },
            },
        },
    }
}
