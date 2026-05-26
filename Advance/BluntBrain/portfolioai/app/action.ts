'use server'

import { z } from "zod"
import { revalidatePath } from "next/cache"
import { prisma } from "@/lib/prisma";



const schema=z.object({
    ticker:z.string().regex(/^[A-Z]{1,5}$/,'Must be 1-5 uppercase letters')
})

type State={error?:string,success?:string}

export async function addToWatchlist(formData:FormData){
    const ticker=formData.get('ticker') as string
    
    await prisma.watchlistItem.create({data:{ticker,userId:'user_123'}})

    revalidatePath('/dashboard')
    
}