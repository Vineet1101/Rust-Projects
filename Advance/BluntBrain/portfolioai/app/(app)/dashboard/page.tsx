import {prisma} from "@/lib/prisma"

export default async function DashboardPage(){
    const items=await prisma.watchlistItem.findMany({
        where:{userId:'user_123'},
        orderBy:{createdAt:'desc'}
    })
    return (<ul>
        {items.map((i)=><li key={i.id}>{i.ticker}</li>)}
    </ul>)
}