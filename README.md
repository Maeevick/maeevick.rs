# Experimental maeevick.com with Rust

I (re)build my website with Rust an experiment with its ecosystem

- [x] Yew: pretty straight forward for a former React / Elm developer.
- [x] Trunk: recommended by Yew's team and pretty cool tool.
- [x] Tailwind: need some Trunk's config and NPX but stays my favorite CSS tool.
- [x] Github Actions: Keep it simple regarding the project nature (website) and the complexity (none).
- [x] Docker: Wrap the application in Docker to prepare next steps.
- [x] Nginx: Serve the application with Nginx to prepare next steps.
    - Current state : 
        - Build: `docker build -t webapp .`
        - Serve: `docker run -it --rm -d -p 8080:80 --name app webapp`
