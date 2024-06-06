package io.github.xpakx.chess.clients;

import io.github.xpakx.chess.clients.event.GameStatus;
import io.github.xpakx.chess.clients.event.UpdateEvent;
import io.github.xpakx.chess.game.GameState;
import org.springframework.amqp.core.AmqpTemplate;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Service;

import java.time.LocalDateTime;

@Service
public class StatePublisher {
    private final AmqpTemplate template;
    private final String updatesTopic;

    public StatePublisher(AmqpTemplate template, @Value("${amqp.exchange.updates}") String updatesTopic) {
        this.template = template;
        this.updatesTopic = updatesTopic;
    }

    public void publish(GameState game, String move) {
        UpdateEvent event = new UpdateEvent();
        event.setGameId(game.getId());
        event.setCurrentState(game.getCurrentState());
        if(!game.isFinished()) {
            event.setStatus(GameStatus.NotFinished);
        } else if(game.isWon()) {
            event.setStatus(GameStatus.Won);
        } else if(game.isLost()) {
            event.setStatus(GameStatus.Lost);
        } else {
            event.setStatus(GameStatus.Drawn);
        }
        event.setUserTurn(game.isFirstUserTurn());
        event.setTimestamp(LocalDateTime.now());

        event.setMove(move);

        template.convertAndSend(updatesTopic, "update", event);
    }
}
