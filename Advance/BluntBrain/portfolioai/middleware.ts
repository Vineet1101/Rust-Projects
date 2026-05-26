import {clerkMiddleware,createRouteMatcher} from "@clerk/nextjs/server"

const isProtectedRoute=createRouteMatcher([
    '/dashboard(.*)',
    '/stocks(,*)'
])

export default clerkMiddleware((auth,req)=>{
    if (isProtectedRoute(req)){
        auth.protect()
    }
})


export const config={
    matcher:[
          '/((?!_next|[^?]*\\.(?:html?|css|js|png|jpg|jpeg|svg|ico)).*)',
    '/(api|trpc)(.*)',
    ]
}