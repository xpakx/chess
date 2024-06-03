package io.github.xpakx.chess.routes;

import org.springframework.beans.factory.annotation.Value;
import org.springframework.cloud.gateway.route.RouteLocator;
import org.springframework.cloud.gateway.route.builder.RouteLocatorBuilder;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

@Configuration
public class RouteLocatorConfig {
    @Bean
    public RouteLocator myRoutes(
            RouteLocatorBuilder builder,
            @Value("${main.url}") final String mainUrl,
            @Value("${game.url}") final String gameUrl
    ) {
        return builder.routes()
                .route("main", r -> r
                        .path("/authenticate", "/register", "/refresh", "/game/**")
                        .uri(mainUrl))
                .route("game", r -> r
                        .path("/app/**", "/topic/**", "/play/**")
                        .uri(gameUrl))
                .build();
    }
}
