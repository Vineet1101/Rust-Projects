'use client'

import { useState } from "react"
export default function PriceDisplay({params}:{params:{price:number,change:number}}){
    
    const [view,changeView]=useState()
    return(
        <div>
            <p>Price: ${params.price}</p>
            <p>Change: ${params.change}</p>
        </div>
    )
}