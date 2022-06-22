use castle_api::types::State;
use mongodb::bson::oid::ObjectId;

use super::Card;

pub struct Organisation {
    _id: ObjectId,
    name: String,
}

struct CreateOrganisation {
    name: String,
    users: Vec<OrganisationMember>,
}

struct OrganisationMember {
    user: ObjectId,
    role: OrganisationRole,
}

enum OrganisationRole {
    Admin,
    Member,
}

impl Organisation {
    // directive authenticated
    async fn create_organisation(
        org: CreateOrganisation,
        state: State,
    ) -> Result<ObjectId, String> {
        todo!()
    }

    async fn get_organisation(org_id: ObjectId, state: State) -> Result<Self, String> {
        todo!()
    }

    async fn add_member(org_id: ObjectId, user_id: ObjectId, state: State) -> Result<(), String> {
        todo!()
    }

    // gets or creates a stripe customer id if it doesn't exist
    async fn stripe_customer_id(&self, state: State) -> String {
        todo!()
    }

    async fn members(&self, state: State) -> Vec<OrganisationMember> {
        todo!()
    }

    // async fn update_plan(&self, product_id: String, ctx: Context) -> Result<(), String> {
    //     todo!()
    // }

    async fn create_subscription(&self, price_id: String, state: State) -> Result<(), String> {
        todo!();
        // let users_len = unimplemented!();
        // let item = json!({
        //     "price": price_id,
        //     "quantity":
        // });
    }

    async fn add_card(&self, token: String, state: State) -> Result<(), String> {
        todo!();
    }

    async fn remove_card(&self, card_id: String, state: State) -> Result<(), String> {
        todo!();
    }

    async fn cards(&self, state: State) -> Result<Vec<Card>, String> {
        todo!();
    }

    async fn default_card(&self, state: State) -> Result<Card, String> {
        todo!();
    }
}
