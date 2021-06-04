use crate::models::NoteItem;

pub struct NoteRepo {
    pub list_notes: Vec<NoteItem>,
}

impl NoteRepo {
    pub fn get_notes(&self) -> &Vec<NoteItem> {
        let list_notes = &self.list_notes;
        list_notes
    }

    pub fn add_note(&mut self, new_note: NoteItem) {
        self.list_notes.push(new_note);
    }
}

pub static mut NOTEREPO: NoteRepo = NoteRepo { list_notes: vec![] };

pub fn init_first_note() {
    let first_note = NoteItem {
        note: "First Note".to_string(),
        is_done: true,
        created_at_epoch: 0,
        updated_at_epoch: 0,
    };

    unsafe { NOTEREPO.add_note(first_note) };
}
