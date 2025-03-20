# Dangling pointers and wild pointers

in computer programming are pointers that do not point to a valid object of the appropriate type.

```rs
{
   char *dp = NULL;
   /* ... */
   {
       char c;
       dp = &c;
   }
     /* c falls out of scope */
     /* dp is now a dangling pointer */
}
```
