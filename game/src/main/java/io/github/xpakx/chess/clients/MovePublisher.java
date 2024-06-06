package io.github.xpakx.chess.clients;

import io.github.xpakx.chess.clients.event.AIEvent;
import io.github.xpakx.chess.clients.event.MoveEvent;
import io.github.xpakx.chess.game.GameState;
import org.springframework.amqp.core.AmqpTemplate;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Service;

@Service
public class MovePublisher {
    private final AmqpTemplate template;
    private final String movesTopic;

    public MovePublisher(AmqpTemplate template, @Value("${amqp.exchange.moves}") String movesTopic) {
        this.template = template;
        this.movesTopic = movesTopic;
    }

    public void sendMove(GameState game, String move) {
        MoveEvent event = new MoveEvent();
        event.setGameId(game.getId());
        event.setGameState(game.getCurrentState());
        event.setMove(move);
        event.setNonCapturingMoves(game.getNonCaptureMoves());
        template.convertAndSend(movesTopic, "move", event);
    }

    public void sendAIEvent(GameState game) {
        var event = new AIEvent();
        event.setGameId(game.getId());
        event.setGameState(game.getCurrentState());
        event.setType(game.getAiType());
        event.setNonCapturingMoves(game.getNonCaptureMoves());
        template.convertAndSend(movesTopic, "ai", event);
    }
}