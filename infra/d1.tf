resource "cloudflare_d1_database" "this" {
  account_id = var.cloudflare_account_id
  name       = "notwithouthelp"
}

resource "cloudflare_d1_database" "this_dev" {
  account_id = var.cloudflare_account_id
  name       = "notwithouthelp-dev"
}
