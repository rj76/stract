// Stract is an open source web search engine.
// Copyright (C) 2023 Stract ApS
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use aes_gcm::{aead::OsRng, Aes256Gcm, KeyInit};

pub mod local;

/// This is currently not used, but let's keep it around for now
/// in case we want to use it later.
pub mod openai;

pub fn generate_key() {
    let key = Aes256Gcm::generate_key(OsRng);
    println!("{}", base64::encode(key.as_slice()));
}