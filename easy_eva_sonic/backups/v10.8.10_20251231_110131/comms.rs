use octocrab::Octocrab;
use octocrab::params::State as ParamState; // Für die Suche
use octocrab::models::IssueState;          // Für das Update
use anyhow::Result;

pub struct GitHubReporter {
    octo: Octocrab,
    repo_owner: String,
    repo_name: String,
}

impl GitHubReporter {
    pub fn new(token: String) -> Result<Self> {
        let octo = Octocrab::builder().personal_token(token).build()?;
        Ok(Self {
            octo,
            repo_owner: "adnan19825".to_string(),
            repo_name: "Easy-Eva-Studio-v7.5".to_string(),
        })
    }
    
    pub async fn create_security_alert(&self, verdict: &str, score: u8, details: &str) -> Result<String> {
        let title = format!("🚨 [NODE-ALERT] {}", verdict);
        let body = format!("### 🛡️ Security Report\n\n**Status:** `{}`\n**Score:** `{}`\n\n**Details:**\n```\n{}\n```\n\n---\n*v10.5 Active Monitoring*", verdict, score, details);
        
        // Octocrab v0.38 erwartet hier explizit Into<Option<Vec<String>>>
        let labels: Option<Vec<String>> = Some(vec!["automated-alert".to_string()]);
        
        let issue = self.octo.issues(&self.repo_owner, &self.repo_name)
            .create(title)
            .body(body)
            .labels(labels) 
            .send()
            .await?;
        Ok(issue.html_url.to_string())
    }

    pub async fn close_all_security_issues(&self) -> Result<()> {
        // Suche nach offenen Issues
        let issues = self.octo.issues(&self.repo_owner, &self.repo_name)
            .list()
            .state(ParamState::Open)
            .send()
            .await?;

        for issue in issues {
            // Wir prüfen manuell, ob unser Label vorhanden ist
            if issue.labels.iter().any(|l| l.name == "automated-alert") {
                println!("   -> Schließe Issue #{}", issue.number);
                self.octo.issues(&self.repo_owner, &self.repo_name)
                    .update(issue.number)
                    .state(IssueState::Closed) // Korrekter Typ für Update
                    .body("✅ Automatisch geschlossen: Bedrohung nicht mehr aktiv.")
                    .send()
                    .await?;
            }
        }
        Ok(())
    }
}

