import Link from "next/link"

export default function DashboardLayout({children}:Readonly<{children:React.ReactNode}>){
    return(
        <div className="flex h-screen bg-gray-950 text-white">
            <aside className="w-60 border-r border-gray-800 p-4">
                <h1 className="text-lg font-semibold mb-6">PortfolioAI</h1>
                <nav className="flex flex-col gap-2 text-sm text-gray-300">
                    <Link href="/dashboard">Watchlist</Link>
                    <Link href="/dashboard/Portfolio">Portfolio</Link>
                    <Link href="/dashboard/discover">Discover</Link>
                </nav>
            </aside>
            <main className="flex-1 overflow-auto p-6">{children}</main>
        </div>
    )
}