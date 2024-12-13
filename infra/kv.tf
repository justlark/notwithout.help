resource "cloudflare_workers_kv_namespace" "prod" {
  account_id = var.cloudflare_account_id
  title      = "notwithouthelp"
}

resource "cloudflare_workers_kv_namespace" "dev" {
  account_id = var.cloudflare_account_id
  title      = "notwithouthelp-dev"
}
