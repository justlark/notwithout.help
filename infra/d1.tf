resource "cloudflare_d1_database" "prod" {
  account_id = var.cloudflare_account_id
  name       = "notwithouthelp"
}

resource "cloudflare_d1_database" "dev" {
  account_id = var.cloudflare_account_id
  name       = "notwithouthelp-dev"
}
