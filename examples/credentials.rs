use adrive_api_rs::objects::credential::Credentials;
use chrono::{DateTime, Utc};

#[tokio::main]
async fn main() {
    let mut cred = Credentials {
        access_token: String::from("eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiI2MzhjNDAxODY2Mzk0ZDQ0YmM5MGRmZjM1YzM5Y2I5MiIsImN1c3RvbUpzb24iOiJ7XCJjbGllbnRJZFwiOlwiMjVkelgzdmJZcWt0Vnh5WFwiLFwiZG9tYWluSWRcIjpcImJqMjlcIixcInNjb3BlXCI6W1wiRFJJVkUuQUxMXCIsXCJTSEFSRS5BTExcIixcIkZJTEUuQUxMXCIsXCJVU0VSLkFMTFwiLFwiVklFVy5BTExcIixcIlNUT1JBR0UuQUxMXCIsXCJTVE9SQUdFRklMRS5MSVNUXCIsXCJCQVRDSFwiLFwiT0FVVEguQUxMXCIsXCJJTUFHRS5BTExcIixcIklOVklURS5BTExcIixcIkFDQ09VTlQuQUxMXCIsXCJTWU5DTUFQUElORy5MSVNUXCIsXCJTWU5DTUFQUElORy5ERUxFVEVcIl0sXCJyb2xlXCI6XCJ1c2VyXCIsXCJyZWZcIjpcImh0dHBzOi8vd3d3LmFsaXl1bmRyaXZlLmNvbS9cIixcImRldmljZV9pZFwiOlwiYzI4MDVhZWU3ODA3NDQ0NzkyZDFlZjE0NGQ2YjhlYjVcIn0iLCJleHAiOjE2ODExNDM1NDQsImlhdCI6MTY4MTEzNjI4NH0.q_xx3iSua9TWtaHFVAICKixO26WUyaAFbhuAvCnPfVPzLVp6jm3kRqdBmSrYU7j-Awn1rScdGhNtg3JnrsuoLzUEOBBAvsaiGvbJ88QP-3aRiQpng4Lv-DI17iug2hErI11Z6lYi-9bZm_Gl4b6hPDlK3G5LqPVMib9t4LkD-1w"),
        refresh_token: String::from("c2805aee7807444792d1ef144d6b8eb5"),
        expire_in: 7200,
        expire_time: "2023-04-08T16:19:04Z".parse::<DateTime<Utc>>().unwrap(),
        drive_id: String::from("1270580"),
        sbox_id: String::from("1270581")
    };
    cred.dump().unwrap();
    cred.refresh_if_needed().await.unwrap();
}
