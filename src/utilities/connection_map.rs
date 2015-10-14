// Copyright 2015 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0.  This, along with the
// Licenses can be found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

const _MAX_ENTRIES : usize = 100;

/// validates that the type can be validated with a public id
pub trait Identifiable {
    /// should return true if the public id is the validator for the type
    fn valid_public_id(&self, public_id: &::public_id::PublicId) -> bool;
}

/// ConnectionMap maintains a double directory to look up a connection or a name V
#[allow(unused)]
pub struct ConnectionMap<V> {
    connection_map: ::std::collections::BTreeMap<V, ::public_id::PublicId>,
    lookup_map: ::std::collections::HashMap<::crust::Connection, V>,
}

#[allow(unused)]
impl<V> ConnectionMap<V> where V: Ord + Clone + Identifiable + ::std::fmt::Debug {
    /// Create a new connection map
    pub fn new() -> ConnectionMap<V> {
        ConnectionMap {
            connection_map: ::std::collections::BTreeMap::new(),
            lookup_map: ::std::collections::HashMap::new(),
        }
    }

    /// Returns true if the identifier is unique in the map
    /// and the connection can be added.  Returns false if the public id is not valid
    /// for the given identifier, or if either the connection or
    /// the identifier already are registered.
    pub fn add_peer(&mut self, connection: ::crust::Connection, identifier: V,
        public_id: ::public_id::PublicId) -> bool {
        if !identifier.valid_public_id(&public_id) { return false; };
        if self.lookup_map.contains_key(&connection) { return false; };
        match self.connection_map.get(&identifier) {
            Some(stored_public_id) => { if stored_public_id != &public_id { return false; }},
            None => {},
        };
        if self.lookup_map.len() > _MAX_ENTRIES { warn!("Exceeded maximum number of connections \
            {:?} when adding {:?} on {:?}", self.lookup_map.len(), identifier, connection); };
        let old_value = self.lookup_map.insert(connection, identifier.clone());
        debug_assert!(old_value.is_none(), "Already verified above the lookup_map does \
            not contain the connection; old identifier is {:?}", identifier);
        let old_value = self.connection_map.insert(identifier, public_id);
        debug_assert!(old_value.is_none(), "Already verified above the connection map does \
            not contain the identifier");
        true
    }

    /// Removes the provided connection and returns the Peer this connection was registered to,
    /// otherwise returns None.
    pub fn drop_connection(&mut self, connection_to_drop: &::crust::Connection) -> Option<::public_id::PublicId> {
        match self.lookup_map.remove(connection_to_drop) {
            Some(identity) => self.connection_map.remove(&identity),
            None => None,
        }
    }

    /// Removes the given identity from the connection map if it exists, returning the public id
    /// and the connection.
    pub fn drop_identity(&mut self, identity: &V)
        -> (Option<::public_id::PublicId>, Vec<::crust::Connection>) {
        let public_id = self.connection_map.remove(identity);
        let connections = self.lookup_map.iter()
            .filter_map(|(c, i)| if i == identity { Some(c.clone()) } else { None })
            .collect::<Vec<::crust::Connection>>();
        (public_id, connections)
    }
}
