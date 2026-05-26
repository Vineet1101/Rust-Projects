import {PrismaClient} from "../generated/prisma/client"

const prisma=new PrismaClient()


async function main(){
    const user=await prisma.user.upsert({
        where: {email:'demo@example.com'},
        update:{},
        create:{
            email:'demo@example.com',
            clerkId:'demo_clerk_id',
            watchlist:{
                create:[
                    {ticker:'AAPL'},
                    {ticker:'NVDA'},
                    {ticker:'TSLA'}
                ]
            }
        }
    })
    console.log('seeded user:',user.email);
    
}


main().finally(()=>prisma.$disconnect())