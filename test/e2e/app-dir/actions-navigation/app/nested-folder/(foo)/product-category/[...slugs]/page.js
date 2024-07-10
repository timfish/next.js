'use client'
import { useFormStatus } from 'react-dom'
import { addToCart } from './actions'

function SubmitButton() {
  const { pending } = useFormStatus()

  return (
    <button type="submit" aria-disabled={pending} id="submit">
      Add to cart
    </button>
  )
}

export default function Page() {
  return (
    <>
      <h1>Add to cart</h1>
      <form action={addToCart}>
        <SubmitButton />
      </form>
    </>
  )
}
