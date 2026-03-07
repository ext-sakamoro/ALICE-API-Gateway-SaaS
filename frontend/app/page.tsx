import Link from 'next/link';
export default function Home() {
  return (
    <div className="min-h-screen bg-background">
      <header className="border-b border-border">
        <div className="max-w-6xl mx-auto px-6 py-4 flex justify-between items-center">
          <h1 className="text-xl font-bold">ALICE API Gateway</h1>
          <div className="flex gap-3">
            <Link href="/auth/login" className="px-4 py-2 text-sm text-muted-foreground hover:text-foreground">Sign in</Link>
            <Link href="/auth/register" className="px-4 py-2 bg-primary text-primary-foreground rounded-md text-sm font-medium hover:opacity-90">Get Started</Link>
          </div>
        </div>
      </header>
      <main>
        <section className="max-w-4xl mx-auto px-6 py-24 text-center space-y-6">
          <h2 className="text-5xl font-bold tracking-tight">Don&apos;t build a gateway.<br />Deploy one in seconds.</h2>
          <p className="text-xl text-muted-foreground max-w-2xl mx-auto">Managed API gateway powered by Rust/Axum. Register APIs, configure rate limits, analyze traffic, and auto-generate OpenAPI specs.</p>
          <div className="flex gap-4 justify-center">
            <Link href="/auth/register" className="px-6 py-3 bg-primary text-primary-foreground rounded-md font-medium hover:opacity-90">Start Free</Link>
            <Link href="#features" className="px-6 py-3 border border-border rounded-md font-medium hover:bg-accent">Learn More</Link>
          </div>
        </section>
        <section id="features" className="max-w-5xl mx-auto px-6 py-16 grid grid-cols-1 md:grid-cols-3 gap-8">
          {[
            { t: 'Low-Latency Proxy', d: 'Rust/Axum reverse proxy with sub-millisecond overhead. DashMap concurrent rate limiting.' },
            { t: 'Traffic Analytics', d: 'Real-time request counts, latency percentiles (p50/p95/p99), error rates, and endpoint breakdown.' },
            { t: 'Auto OpenAPI', d: 'Automatically generate OpenAPI 3.1 specs from your registered APIs. Always up-to-date documentation.' },
          ].map((f) => (
            <div key={f.t} className="border border-border rounded-lg p-6 space-y-2">
              <h3 className="font-semibold text-lg">{f.t}</h3>
              <p className="text-sm text-muted-foreground">{f.d}</p>
            </div>
          ))}
        </section>
      </main>
      <footer className="border-t border-border py-8 text-center text-sm text-muted-foreground">AGPL-3.0 | ALICE API Gateway SaaS</footer>
    </div>
  );
}
