# nemo

an attempt at writing a database

## design

```text 
                   input loop                                  
                     │   │                                     
                     │   │                                     
command gets handled◄┘   └─►sql get handled                    
                                │                              
                                ▼                              
                            tokenise sql                       
                                │                              
                                ▼                              
                            parse sql                          
                                │                              
                                ▼                              
                            pass ast from parser to data engine
                                │                              
                                ▼                              
                            run ast on data                    
                                │                              
                                ▼                              
                            return the output                  
```