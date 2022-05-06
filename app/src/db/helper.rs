use mongodb::bson::oid::ObjectId;
use mongodb::bson::Bson;

pub fn bson_object_id_from_str(data: impl AsRef<str>) -> Result<Bson, mongodb::bson::oid::Error> {
    Ok(Bson::ObjectId(ObjectId::parse_str(data)?))
}
