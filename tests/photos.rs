#![allow(unused_variables)]
#[macro_use]
extern crate cargonauts;


use cargonauts::api;

routes! {
    resource User: [get, patch] {
        has many Photo: [fetch, append];
    }
    resource Photo: [get, index, post, delete] {
        has one User: [fetch];
    }
}

struct User;

impl cargonauts::Serialize for User {
    fn serialize<S: cargonauts::Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        unimplemented!()
    }
}

impl api::Resource for User {
    type Id = u32;
    type Repr = User;

    fn id(&self) -> u32 { unimplemented!() }
    fn repr(self) -> User { self }

    fn resource() -> &'static str { "user" }
    fn resource_plural() -> &'static str { "users" }
}

impl api::Get for User {
    fn get(id: &u32) -> api::Result<User> {
        unimplemented!()
    }
}

impl api::Patch for User {
    type Patch = ();
    fn patch(id: &u32, patch: Self::Patch) -> api::Result<User> {
        unimplemented!()
    }
}

impl api::rel::HasMany<Photo> for User {
    fn has_many(entity: &api::Entity<User>) -> api::Result<Vec<u32>> {
        unimplemented!()
    }
}

impl api::rel::AppendLinks<Photo> for User {
    fn append_links(entity: &api::Entity<User>, rel_ids: &[u32]) -> api::Result<()> {
        unimplemented!()
    }
}

struct Photo;

impl cargonauts::Serialize for Photo {
    fn serialize<S: cargonauts::Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        unimplemented!()
    }
}

impl cargonauts::Deserialize for Photo {
    fn deserialize<D: cargonauts::Deserializer>(deserializer: &mut D) -> Result<Self, D::Error> {
        unimplemented!()
    }
}

impl api::Resource for Photo {
    type Id = u32;
    type Repr = Photo;

    fn id(&self) -> u32 { unimplemented!() }
    fn repr(self) -> Photo { self }

    fn resource() -> &'static str { "photo" }
    fn resource_plural() -> &'static str { "photos" }
}

impl api::Delete for Photo {
    fn delete(id: &u32) -> api::Result<()> {
        unimplemented!()
    }
}

impl api::Get for Photo {
    fn get(id: &u32) -> api::Result<Photo> {
        unimplemented!()
    }
}

impl api::Index for Photo {
    fn index() -> api::Result<Vec<Photo>> {
        unimplemented!()
    }
}

impl api::Post for Photo {
    fn post(repr: Photo) -> api::Result<Photo> {
        unimplemented!()
    }
}

impl api::rel::HasOne<User> for Photo {
    fn has_one(entity: &api::Entity<Photo>) -> api::Result<Option<u32>> {
        unimplemented!()
    }
}

#[test]
fn it_compiles() { }

#[test]
fn it_has_attached_routes() {
    use cargonauts::router::mock::MockRouter;
    
    const USERS_ROUTES: &'static [&'static str] = &["get", "patch"];
    const PHOTOS_ROUTES: &'static [&'static str] = &["get", "index", "post", "delete"];
    const USERS_PHOTOS_ROUTES: &'static [&'static str] = &["fetch-one", "fetch-rel", "append", "append-links"];
    const PHOTOS_USER_ROUTES: &'static [&'static str] = &["fetch-many", "fetch-rel"];

    let mut router = MockRouter::new();
    attach_routes(&mut router);

    assert_eq!(&router.methods_for("users")[..], USERS_ROUTES);
    assert_eq!(&router.methods_for("photos")[..], PHOTOS_ROUTES);
    assert_eq!(&router.methods_for_rel("users", "photos")[..], USERS_PHOTOS_ROUTES);
    assert_eq!(&router.methods_for_rel("photos", "user")[..], PHOTOS_USER_ROUTES);
}
